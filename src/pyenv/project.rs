use anyhow::{bail, Error, Result};
use pep508_rs::Requirement;
use scraper::{Html, Selector};
use std::fs::File;
use std::path::PathBuf;
use url::Url;

use super::download::{download, fetch_text};
use super::installer::Installer;
use super::link::{parse_link_from_url, PackageLink};
use crate::pyenv::checksum;
use crate::pyenv::utils::canonicalize_name;

use super::config::PyPIMirror;

pub struct ProjectIndex {
    pypi: PyPIMirror,
    project_name: String,
    project_url: String,
    canonical_name: String,
    links: Vec<PackageLink>,
}

impl ProjectIndex {
    pub fn new(pypi: &PyPIMirror, project_name: &str) -> Self {
        let canonical_name = canonicalize_name(project_name);

        ProjectIndex {
            pypi: pypi.clone(),
            project_name: project_name.to_string(),
            project_url: pypi.package_url(canonical_name.as_str()),
            canonical_name,
            links: Vec::new(),
        }
    }

    pub fn pypi(&self) -> &PyPIMirror {
        &self.pypi
    }

    pub fn package_name(&self) -> &str {
        &self.project_name
    }

    pub fn project_url(&self) -> &str {
        &self.project_url
    }
}

pub async fn download_requirement(
    installer: &Installer,
    pypi: &PyPIMirror,
    requirement: &Requirement,
) -> Result<()> {
    let project_name = requirement.name.as_str();

    let mut project_index = ProjectIndex::new(pypi, project_name);

    let page = fetch_text(
        installer,
        project_index.project_url(),
        format!("从{}获取{}包信息", pypi.name(), project_name).as_str(),
    )
    .await?;

    parse_index_html_page(&mut project_index, page.as_str())?;

    let candidates = find_candidates_links(installer, &project_index, &requirement)?;

    // println!("需求：{}", requirement);
    // for link in find_candidates_links(installer, &project_index, &requirement)? {
    //     println!("link: {} {}", link.package_version(), link.filename_base());
    // }

    let link = if candidates.len() > 0 {
        candidates[0]
    } else {
        bail!(
            "未在{}发现满足需求({})的包: {}",
            pypi.name(),
            requirement,
            project_index.project_url()
        )
    };

    let cached_filename = &installer.cached_packages_dir.join(link.file_name());

    if is_cached_file_available(link, cached_filename)? {
        installer.log(format!("程序包{}本地已缓存，无需下载", cached_filename.display()).as_str());
        return Ok(());
    }

    download_link(installer, pypi, link, cached_filename).await?;

    Ok(())
}

async fn download_link(
    installer: &Installer,
    pypi: &PyPIMirror,
    link: &PackageLink,
    cached_filename: &PathBuf,
) -> Result<()> {
    let buffer = download(
        installer,
        link.url(),
        &format!("从{}下载{}", pypi.name(), link.file_name()),
    )
    .await?;

    if let Some((checksum_method, hexcode)) = link.checksum() {
        if !checksum(checksum_method, &buffer, hexcode)? {
            bail!("文件sha256完整检查失败: {}", link.file_name())
        }
    } else {
        bail!("文件{}无checksum码", cached_filename.display())
    }

    let mut package_file = File::create(cached_filename)?;

    use std::io::Write;
    package_file.write_all(&buffer)?;

    Ok(())
}

fn is_cached_file_available(link: &PackageLink, cached_filename: &PathBuf) -> Result<bool> {
    if !cached_filename.is_file() {
        // 文件不存在，无需进一步的检查
        return Ok(false);
    }

    // 文件已经存在，检查是否完整
    let _err: Option<anyhow::Error> = match std::fs::read(cached_filename) {
        Ok(buf) => {
            if let Some((checksum_method, hexcode)) = link.checksum() {
                match checksum(checksum_method, &buf, hexcode) {
                    Ok(true) => return Ok(true),
                    Ok(false) => None,
                    Err(err) => Some(err),
                }
            } else {
                None
            }
        }
        Err(err) => Some(err.into()),
    };

    if let Err(err) = std::fs::remove_file(cached_filename) {
        bail!("删除无效临时文件{}错误: {}", cached_filename.display(), err)
    }

    Ok(false)
}

fn parse_index_html_page(
    project_index: &mut ProjectIndex,
    html_content: &str,
) -> Result<(), Error> {
    let document = Html::parse_document(html_content);
    let link_selector = Selector::parse("body > a").unwrap();
    let base_selector = Selector::parse("base").unwrap();

    let base_url = Url::parse(match document.select(&base_selector).next() {
        Some(node) => {
            //
            node.value()
                .attr("href")
                .unwrap_or(project_index.project_url())
        }
        None => project_index.project_url(),
    })?;

    // let mut links = Vec::new();
    for node in document.select(&link_selector) {
        let elem = node.value();
        let href = match elem.attr("href") {
            Some(href) => href,
            None => continue,
        };

        let url = base_url.join(href)?;
        let requires_python = elem.attr("data-requires-python");
        let yanked_reason = elem.attr("data-yanked");

        let Some(link) = parse_link_from_url(
            &project_index.canonical_name,
            url,
            requires_python,
            yanked_reason,
        )?
        else {
            continue;
        };

        // let file_ext = link.filename_extension();
        // println!("{:?}\n", link);

        project_index.links.push(link);
    }

    Ok(())

    // Ok(project_index)
}

pub fn find_candidates_links<'a>(
    installer: &Installer,
    // target_env: &TargetEnv,
    index: &'a ProjectIndex,
    requirement: &Requirement,
) -> Result<Vec<&'a PackageLink>, Error> {
    use pep440_rs::{Version, VersionSpecifiers};
    use pep508_rs::VersionOrUrl;

    use std::str::FromStr;

    let python_version = match Version::from_str(&installer.python_version_full) {
        Ok(version) => version,
        Err(err) => bail!("parsing version: {}", err),
    };

    let pkg_specifiers = match &requirement.version_or_url {
        Some(VersionOrUrl::VersionSpecifier(pkg_specifiers)) => pkg_specifiers,
        Some(VersionOrUrl::Url(url)) => bail!("不支持直接给链接下载: {}", url),
        None => bail!("requirement: None"),
    };

    let mut candidates: Vec<(Version, Option<u32>, &PackageLink)> = Vec::new();
    for link in &index.links {
        // 检查连接是否满足当前环境的Python版本需求
        if let Some(requires_python) = link.requires_python() {
            let specifiers = match VersionSpecifiers::from_str(requires_python) {
                Ok(specifiers) => specifiers,
                Err(_err) => {
                    // 存在'>=3.4.*'这样不合规范的
                    // bail!("parsing specifiers: {}", err);
                    continue;
                }
            };

            if !specifiers.contains(&python_version) {
                continue; // 不满足Python版本需求
            }
        };

        // 检查包的版本是否满足需求
        let Ok(pkg_version) = Version::from_str(link.package_version()) else {
            bail!("parsing package version: '{}'", link.package_version());
        };

        if !pkg_specifiers.contains(&pkg_version) {
            continue;
        }

        // 匹配环境最合适的tag
        if link.is_wheel() {
            let Some(best_tag_rank) = get_best_tag_rank(installer, link) else {
                continue; // 无适合的tag
            };

            candidates.push((pkg_version, Some(best_tag_rank), link));
        } else {
            candidates.push((pkg_version, None, link));
        }
    }

    candidates.sort_by(|a, b| {
        // 从大到小排列version，从小到大排列tag的rank
        use std::cmp::Ordering::{Equal, Greater, Less};
        match a.0.cmp(&b.0) {
            Less => Greater,
            Equal => match (a.1, b.1) {
                (None, None) => Equal,
                (None, Some(_)) => Greater,
                (Some(_), None) => Less,
                (Some(t_a), Some(t_b)) => t_a.cmp(&t_b),
            },
            Greater => Less,
        }
    });

    Ok(candidates.iter().map(|x| x.2).collect())
}

pub fn get_best_tag_rank(installer: &Installer, link: &PackageLink) -> Option<u32> {
    let Some(tags) = link.wheel_tags() else {
        return None;
    };

    let mut ranks = Vec::new();
    for tag in tags {
        if let Some(i) = installer.support_tags_map.get(tag.as_str()) {
            ranks.push(*i);
        }
    }

    ranks.iter().min().copied()
}
