use anyhow::{bail, Error, Result};
use pep508_rs::Requirement;
use scraper::{Html, Selector};
use url::Url;

use super::super::installer::Installer;
use super::link::{self, parse_link_from_url, PackageLink};
use crate::pyenv::utils::canonicalize_name;

pub struct ProjectIndex {
    index_url: String,
    project_name: String,
    project_url: String,
    canonical_name: String,
    links: Vec<PackageLink>,
}

impl ProjectIndex {
    pub fn new(index_url: &str, project_name: &str) -> Self {
        let canonical_name = canonicalize_name(project_name);

        let project_url = if index_url.ends_with('/') {
            format!("{}{}/", index_url, canonical_name)
        } else {
            format!("{}/{}/", index_url, canonical_name)
        };

        ProjectIndex {
            index_url: index_url.to_string(),
            project_name: project_name.to_string(),
            project_url,
            canonical_name,
            links: Vec::new(),
        }
    }

    pub fn project_url(&self) -> &str {
        &self.project_url
    }
}

pub async fn download_requirement(installer: &Installer, requirement: &Requirement) -> Result<()>{

    // let requirements = get_requirements(&installer).await?;

    Ok(())
}

async fn get_project_index(installer: &Installer, requirement: &Requirement) -> Result<()> {
    let project_name = requirement.name.as_str();

    let index_url = "https://pypi.tuna.tsinghua.edu.cn/simple";

    let mut project_index = ProjectIndex::new(index_url, project_name);

    // use std::str::FromStr;
    // let requirement = "torch~=2.0.0";
    // let requirement = Requirement::from_str(requirement)?;

    println!(
        "package-name: {}, extras: {:?}, python_version: {:?}",
        requirement.name, requirement.extras, requirement.marker,
    );

    // let resp = client.get(url).send().await?;
    // let page = resp.text().await?;

    parse_index_html_page(&mut project_index, PAGE_CONTENT2)?;

    // let candidates = find_candidates_links(installer, &project_index, &requirement)?;

    for link in find_candidates_links(installer, &project_index, &requirement)? {
        println!("link: {} {}", link.package_version(), link.filename_base());
    }

    Ok(())
}

// pub async fn get_project_index(client: &reqwest::Client) -> Result<(), Error> {
//     let project_name = "jupyterlab";
//     let target_env = TargetEnv::new();

//     let project_url = "https://pypi.tuna.tsinghua.edu.cn/simple/jupyterlab/";
//     // let resp = client.get(url).send().await?;
//     // let page = resp.text().await?;

//     let mut project_index = ProjectIndex {
//         project_name: project_name.to_string(),
//         canonical_name: canonicalize_name(project_name),
//         links: Vec::new(),
//     };

//     use std::str::FromStr;

//     let requirement = "torch~=2.0.0";
//     let requirement = Requirement::from_str(requirement)?;

//     println!(
//         "package-name: {}, extras: {:?}, python_version: {:?}",
//         requirement.name, requirement.extras, requirement.marker,
//     );

//     parse_index_html_page(&mut project_index, project_url, PAGE_CONTENT2)?;
//     let candidates = find_candidates_links(&target_env, &project_index, &requirement)?;

//     for link in candidates {
//         println!("link: {} {}", link.package_version(), link.filename_base());
//     }

//     Ok(())
// }

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

        let link = parse_link_from_url(
            &project_index.canonical_name,
            url,
            requires_python,
            yanked_reason,
        )?;

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
                Err(err) => bail!("parsing specifiers: {}", err),
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

static PAGE_CONTENT: &str = r#"
<!DOCTYPE html>
<html>
  <head>
    <meta name="pypi:repository-version" content="1.0">
    <title>Links for jupyterlab</title>
  </head>
  <body>
    <h1>Links for jupyterlab</h1>
    <a href="../../packages/db/1d/a4645d6385cd03aa7703a8695317b431e92460a5a9168b9fac59d2aafd60/jupyterlab-4.0.4-py3-none-any.whl#sha256=23eef35d22be8f2ad9b873ec41ceb2e8c3b0dc8ae740c0f973e2de09e587530f" data-requires-python="&gt;=3.8">jupyterlab-4.0.4-py3-none-any.whl</a><br/>
    <a href="../../packages/94/86/3543bfd8873a4ff7e119acf5430a699ab5cbab243fee16423aee44574c35/jupyterlab-4.0.4.tar.gz#sha256=049449a56d93202ed204e0e86f96f5a3447a08cfc09fb012fd239e178651cb34" data-requires-python="&gt;=3.8">jupyterlab-4.0.4.tar.gz</a><br/>
    <a href="../../packages/71/a3/38b9d6492a393dcfdae9e82021655432a9fd6e8f4c03c30a7b55036c0d70/jupyterlab-4.0.5-py3-none-any.whl#sha256=13b3a326e7b95d72746fe20dbe80ee1e71165d6905e01ceaf1320eb809cb1b47" data-requires-python="&gt;=3.8">jupyterlab-4.0.5-py3-none-any.whl</a><br/>
    <a href="../../packages/4a/fe/940531a702a6b1e05dadf98ec9f34bb159e1c1b47253ce7bdfbaa54c262b/jupyterlab-4.0.5.tar.gz#sha256=de49deb75f9b9aec478ed04754cbefe9c5d22fd796a5783cdc65e212983d3611" data-requires-python="&gt;=3.8">jupyterlab-4.0.5.tar.gz</a><br/>
    <a href="../../packages/3b/43/2368d8ffee6e33f282f548d42fa222bd385cc9f66545b260e7d08e90046b/jupyterlab-4.0.6-py3-none-any.whl#sha256=7d9dacad1e3f30fe4d6d4efc97fda25fbb5012012b8f27cc03a2283abcdee708" data-requires-python="&gt;=3.8">jupyterlab-4.0.6-py3-none-any.whl</a><br/>
    <a href="../../packages/53/e9/6aba4b603ee9c78d1c81bad4112de152bbb078cd77e15377f9f7d8184901/jupyterlab-4.0.6.tar.gz#sha256=6c43ae5a6a1fd2fdfafcb3454004958bde6da76331abb44cffc6f9e436b19ba1" data-requires-python="&gt;=3.8">jupyterlab-4.0.6.tar.gz</a><br/>
</body>
</html>
<!--SERIAL 19753723-->"#;

static PAGE_CONTENT2: &str = r#"
<!DOCTYPE html>
<html>
  <head>
    <meta name="pypi:repository-version" content="1.0">
    <title>Links for torch</title>
  </head>
  <body>
    <h1>Links for torch</h1>
<a href="../../packages/9f/cd/670e5e178db87065ee60f60fb35b040abbb819a1f686a91d9ff799fc5048/torch-2.0.0-1-cp310-cp310-manylinux2014_aarch64.whl#sha256=c9090bda7d2eeeecd74f51b721420dbeb44f838d4536cc1b284e879417e3064a" data-requires-python="&gt;=3.8.0">torch-2.0.0-1-cp310-cp310-manylinux2014_aarch64.whl</a><br/>
<a href="../../packages/21/7a/f43f2f490836dfc2de466dbc86cd75357d9ae3945c084efa290fad15976f/torch-2.0.0-1-cp311-cp311-manylinux2014_aarch64.whl#sha256=bd42db2a48a20574d2c33489e120e9f32789c4dc13c514b0c44272972d14a2d7" data-requires-python="&gt;=3.8.0">torch-2.0.0-1-cp311-cp311-manylinux2014_aarch64.whl</a><br/>
<a href="../../packages/ad/b5/449aa2a51b48dc389b50deae7d9260377a5925e63359cd0dd96d7ebc81a9/torch-2.0.0-1-cp38-cp38-manylinux2014_aarch64.whl#sha256=8969aa8375bcbc0c2993e7ede0a7f889df9515f18b9b548433f412affed478d9" data-requires-python="&gt;=3.8.0">torch-2.0.0-1-cp38-cp38-manylinux2014_aarch64.whl</a><br/>
<a href="../../packages/fa/f4/c90ede3d6ea4dd0f056c11d9d0bdac2408f51ac7de194539453f3f572a51/torch-2.0.0-1-cp39-cp39-manylinux2014_aarch64.whl#sha256=ab2da16567cb55b67ae39e32d520d68ec736191d88ac79526ca5874754c32203" data-requires-python="&gt;=3.8.0">torch-2.0.0-1-cp39-cp39-manylinux2014_aarch64.whl</a><br/>
<a href="../../packages/b6/b1/f562cb533751c272d23f605858cd17d6a6c50fa8cd3c1f99539e2acd359f/torch-2.0.0-cp310-cp310-manylinux1_x86_64.whl#sha256=7a9319a67294ef02459a19738bbfa8727bb5307b822dadd708bc2ccf6c901aca" data-requires-python="&gt;=3.8.0">torch-2.0.0-cp310-cp310-manylinux1_x86_64.whl</a><br/>
<a href="../../packages/47/1f/0213a42f0e290b3057601bd6f03f54712b1c39bdd014fb4d594455503dfa/torch-2.0.0-cp310-cp310-manylinux2014_aarch64.whl#sha256=9f01fe1f6263f31bd04e1757946fd63ad531ae37f28bb2dbf66f5c826ee089f4" data-requires-python="&gt;=3.8.0">torch-2.0.0-cp310-cp310-manylinux2014_aarch64.whl</a><br/>
<a href="../../packages/87/e2/62dbdfc85d3b8f771bc4b1a979ee6a157dbaa8928981dabbf45afc6d13dc/torch-2.0.0-cp310-cp310-win_amd64.whl#sha256=527f4ae68df7b8301ee6b1158ca56350282ea633686537b30dbb5d7b4a52622a" data-requires-python="&gt;=3.8.0">torch-2.0.0-cp310-cp310-win_amd64.whl</a><br/>
<a href="../../packages/c6/20/8200a1c143aca65c72f820a5e7ba4cb3121ad655ad96c5e88395ba381f1f/torch-2.0.0-cp310-none-macosx_10_9_x86_64.whl#sha256=ce9b5a49bd513dff7950a5a07d6e26594dd51989cee05ba388b03e8e366fd5d5" data-requires-python="&gt;=3.8.0">torch-2.0.0-cp310-none-macosx_10_9_x86_64.whl</a><br/>
<a href="../../packages/59/5c/b032a68257189c0b9398bfd7542efa50b3f9a2d08b537167f4c02a69a4b6/torch-2.0.0-cp310-none-macosx_11_0_arm64.whl#sha256=53e1c33c6896583cdb9a583693e22e99266444c4a43392dddc562640d39e542b" data-requires-python="&gt;=3.8.0">torch-2.0.0-cp310-none-macosx_11_0_arm64.whl</a><br/>
<a href="../../packages/83/0b/b83dfba34421cfb1fc41583a479fbeaec0733ec9f59465702997d8de5e10/torch-2.0.0-cp311-cp311-manylinux1_x86_64.whl#sha256=09651bff72e439d004c991f15add0c397c66f98ab36fe60d5514b44e4da722e8" data-requires-python="&gt;=3.8.0">torch-2.0.0-cp311-cp311-manylinux1_x86_64.whl</a><br/>
<a href="../../packages/da/c2/cbac2af26537b82c265a4d53330c540e805185ca2272f33a918a3dedc3a0/torch-2.0.0-cp311-cp311-manylinux2014_aarch64.whl#sha256=d439aec349c98f12819e8564b8c54008e4613dd4428582af0e6e14c24ca85870" data-requires-python="&gt;=3.8.0">torch-2.0.0-cp311-cp311-manylinux2014_aarch64.whl</a><br/>
<a href="../../packages/7f/fd/1438b0c44639d106892b19d386611fefd5add11d339ff623ac7a177b8323/torch-2.0.0-cp311-cp311-win_amd64.whl#sha256=2802f84f021907deee7e9470ed10c0e78af7457ac9a08a6cd7d55adef835fede" data-requires-python="&gt;=3.8.0">torch-2.0.0-cp311-cp311-win_amd64.whl</a><br/>
<a href="../../packages/63/23/b2c6f3ef643c0a9a1d22ed2be9b5fe023d6cd1fe1729d5b03e9a695ab3d7/torch-2.0.0-cp311-none-macosx_10_9_x86_64.whl#sha256=01858620f25f25e7a9ec4b547ff38e5e27c92d38ec4ccba9cfbfb31d7071ed9c" data-requires-python="&gt;=3.8.0">torch-2.0.0-cp311-none-macosx_10_9_x86_64.whl</a><br/>
<a href="../../packages/ee/a9/43610ad590dad7109c3890bf9ffffaea76dab590a0e2cf6d6e957fee4613/torch-2.0.0-cp311-none-macosx_11_0_arm64.whl#sha256=9a2e53b5783ef5896a6af338b36d782f28e83c8ddfc2ac44b67b066d9d76f498" data-requires-python="&gt;=3.8.0">torch-2.0.0-cp311-none-macosx_11_0_arm64.whl</a><br/>
<a href="../../packages/89/5a/0d017d8d45cc309f9de8e5b8edc9b6b204d8c47936a3f2b84cf01650cf98/torch-2.0.0-cp38-cp38-manylinux1_x86_64.whl#sha256=ec5fff2447663e369682838ff0f82187b4d846057ef4d119a8dea7772a0b17dd" data-requires-python="&gt;=3.8.0">torch-2.0.0-cp38-cp38-manylinux1_x86_64.whl</a><br/>
<a href="../../packages/47/af/8266ea35c6a4e8a59b5e348288debdfc7d9a91356dd674b838131546aa6e/torch-2.0.0-cp38-cp38-manylinux2014_aarch64.whl#sha256=11b0384fe3c18c01b8fc5992e70fc519cde65e44c51cc87be1838c1803daf42f" data-requires-python="&gt;=3.8.0">torch-2.0.0-cp38-cp38-manylinux2014_aarch64.whl</a><br/>
<a href="../../packages/49/97/fdb166f3123b4c3017d301e972a9ef10effd050ffc725ba0df6f962176d7/torch-2.0.0-cp38-cp38-win_amd64.whl#sha256=e54846aa63855298cfb1195487f032e413e7ac9cbfa978fda32354cc39551475" data-requires-python="&gt;=3.8.0">torch-2.0.0-cp38-cp38-win_amd64.whl</a><br/>
<a href="../../packages/f7/ee/e00f3fab0383fccf8ee1697ba468e0248bd36a9942d00d6c12fb08cb393a/torch-2.0.0-cp38-none-macosx_10_9_x86_64.whl#sha256=cc788cbbbbc6eb4c90e52c550efd067586c2693092cf367c135b34893a64ae78" data-requires-python="&gt;=3.8.0">torch-2.0.0-cp38-none-macosx_10_9_x86_64.whl</a><br/>
<a href="../../packages/67/14/f4b5fb08f3fe59c610e07daa798d194dc40158b2011229dea7e7f5ab182b/torch-2.0.0-cp38-none-macosx_11_0_arm64.whl#sha256=d292640f0fd72b7a31b2a6e3b635eb5065fcbedd4478f9cad1a1e7a9ec861d35" data-requires-python="&gt;=3.8.0">torch-2.0.0-cp38-none-macosx_11_0_arm64.whl</a><br/>
<a href="../../packages/5f/24/16e94ac3a470027a2f6cf56dbbe2ce1b2742fa0ac98844f039fad103e142/torch-2.0.0-cp39-cp39-manylinux1_x86_64.whl#sha256=6befaad784004b7af357e3d87fa0863c1f642866291f12a4c2af2de435e8ac5c" data-requires-python="&gt;=3.8.0">torch-2.0.0-cp39-cp39-manylinux1_x86_64.whl</a><br/>
<a href="../../packages/36/60/aa7bf18070611e7b019886d34516337ce6a2fe9da60745bc90b448642a10/torch-2.0.0-cp39-cp39-manylinux2014_aarch64.whl#sha256=a83b26bd6ae36fbf5fee3d56973d9816e2002e8a3b7d9205531167c28aaa38a7" data-requires-python="&gt;=3.8.0">torch-2.0.0-cp39-cp39-manylinux2014_aarch64.whl</a><br/>
<a href="../../packages/25/45/b91c4bf6b4b6325e9c758ef1203978ae5455c71e52054a7aca23befe33df/torch-2.0.0-cp39-cp39-win_amd64.whl#sha256=c7e67195e1c3e33da53954b026e89a8e1ff3bc1aeb9eb32b677172d4a9b5dcbf" data-requires-python="&gt;=3.8.0">torch-2.0.0-cp39-cp39-win_amd64.whl</a><br/>
<a href="../../packages/d7/55/fd0f058d5d7d912a0a360cbb1fbf60940c0589eaa0cc08bcd530cb08b86e/torch-2.0.0-cp39-none-macosx_10_9_x86_64.whl#sha256=6e0b97beb037a165669c312591f242382e9109a240e20054d5a5782d9236cad0" data-requires-python="&gt;=3.8.0">torch-2.0.0-cp39-none-macosx_10_9_x86_64.whl</a><br/>
<a href="../../packages/4d/80/760f3edcf0179c3111fae496b97ee3fa9171116b4bccae6e073efe928e72/torch-2.0.0-cp39-none-macosx_11_0_arm64.whl#sha256=297a4919aff1c0f98a58ebe969200f71350a1d4d4f986dbfd60c02ffce780e99" data-requires-python="&gt;=3.8.0">torch-2.0.0-cp39-none-macosx_11_0_arm64.whl</a><br/>
<a href="../../packages/8c/4d/17e07377c9c3d1a0c4eb3fde1c7c16b5a0ce6133ddbabc08ceef6b7f2645/torch-2.0.1-cp310-cp310-manylinux1_x86_64.whl#sha256=8ced00b3ba471856b993822508f77c98f48a458623596a4c43136158781e306a" data-requires-python="&gt;=3.8.0">torch-2.0.1-cp310-cp310-manylinux1_x86_64.whl</a><br/>
<a href="../../packages/21/33/4925decd863ce88ed9190a4bd872b01c146243ee68db08c72923984fe335/torch-2.0.1-cp310-cp310-manylinux2014_aarch64.whl#sha256=359bfaad94d1cda02ab775dc1cc386d585712329bb47b8741607ef6ef4950747" data-requires-python="&gt;=3.8.0">torch-2.0.1-cp310-cp310-manylinux2014_aarch64.whl</a><br/>
<a href="../../packages/8a/e7/c216fe520b877cf4fe03858c825cd2031ca3e81e455b89639c9b5ec91981/torch-2.0.1-cp310-cp310-win_amd64.whl#sha256=7c84e44d9002182edd859f3400deaa7410f5ec948a519cc7ef512c2f9b34d2c4" data-requires-python="&gt;=3.8.0">torch-2.0.1-cp310-cp310-win_amd64.whl</a><br/>
<a href="../../packages/2e/27/5c912ccc490ec78585cd463198e80be27b53db77f02e7398b41305606399/torch-2.0.1-cp310-none-macosx_10_9_x86_64.whl#sha256=567f84d657edc5582d716900543e6e62353dbe275e61cdc36eda4929e46df9e7" data-requires-python="&gt;=3.8.0">torch-2.0.1-cp310-none-macosx_10_9_x86_64.whl</a><br/>
<a href="../../packages/5a/77/778954c0aad4f7901a1ba02a129bca7467c64a19079108e6b1d6ce8ae575/torch-2.0.1-cp310-none-macosx_11_0_arm64.whl#sha256=787b5a78aa7917465e9b96399b883920c88a08f4eb63b5a5d2d1a16e27d2f89b" data-requires-python="&gt;=3.8.0">torch-2.0.1-cp310-none-macosx_11_0_arm64.whl</a><br/>
<a href="../../packages/c8/21/25020cfdd9f564a72f400ee491610e50cb212e8add8031abaa959af6451e/torch-2.0.1-cp311-cp311-manylinux1_x86_64.whl#sha256=e617b1d0abaf6ced02dbb9486803abfef0d581609b09641b34fa315c9c40766d" data-requires-python="&gt;=3.8.0">torch-2.0.1-cp311-cp311-manylinux1_x86_64.whl</a><br/>
<a href="../../packages/5d/61/7273dea60a17c63d9eaef04ae8fee02351e0cb477e76df4ea211896ae124/torch-2.0.1-cp311-cp311-manylinux2014_aarch64.whl#sha256=b6019b1de4978e96daa21d6a3ebb41e88a0b474898fe251fd96189587408873e" data-requires-python="&gt;=3.8.0">torch-2.0.1-cp311-cp311-manylinux2014_aarch64.whl</a><br/>
<a href="../../packages/d0/c8/f0dc8642e3ce0a3ae5f05e5149ab9df5375d569294f7be9a1ab1d95a1d76/torch-2.0.1-cp311-cp311-win_amd64.whl#sha256=dbd68cbd1cd9da32fe5d294dd3411509b3d841baecb780b38b3b7b06c7754434" data-requires-python="&gt;=3.8.0">torch-2.0.1-cp311-cp311-win_amd64.whl</a><br/>
<a href="../../packages/8d/9b/f20686a5ebd09c6feacced771cf4041a521c411c5bb10359580e9e491797/torch-2.0.1-cp311-none-macosx_10_9_x86_64.whl#sha256=ef654427d91600129864644e35deea761fb1fe131710180b952a6f2e2207075e" data-requires-python="&gt;=3.8.0">torch-2.0.1-cp311-none-macosx_10_9_x86_64.whl</a><br/>
<a href="../../packages/85/68/f901437d3e3ef6fe97adb1f372479626d994185b8fa06803f5bdf3bb90fd/torch-2.0.1-cp311-none-macosx_11_0_arm64.whl#sha256=25aa43ca80dcdf32f13da04c503ec7afdf8e77e3a0183dd85cd3e53b2842e527" data-requires-python="&gt;=3.8.0">torch-2.0.1-cp311-none-macosx_11_0_arm64.whl</a><br/>
<a href="../../packages/96/28/026dc037f177d53558477931677b120f649dd5a0dcdc4b44dc38b3d75711/torch-2.0.1-cp38-cp38-manylinux1_x86_64.whl#sha256=5ef3ea3d25441d3957348f7e99c7824d33798258a2bf5f0f0277cbcadad2e20d" data-requires-python="&gt;=3.8.0">torch-2.0.1-cp38-cp38-manylinux1_x86_64.whl</a><br/>
<a href="../../packages/90/f6/b0358e90e10306f80c474379ae1c637760848903033401d3e662563f83a3/torch-2.0.1-cp38-cp38-manylinux2014_aarch64.whl#sha256=0882243755ff28895e8e6dc6bc26ebcf5aa0911ed81b2a12f241fc4b09075b13" data-requires-python="&gt;=3.8.0">torch-2.0.1-cp38-cp38-manylinux2014_aarch64.whl</a><br/>
<a href="../../packages/22/15/b2e3b53bf569579900175626998a927596c59f6a4a9e4f773f1d303efb81/torch-2.0.1-cp38-cp38-win_amd64.whl#sha256=f66aa6b9580a22b04d0af54fcd042f52406a8479e2b6a550e3d9f95963e168c8" data-requires-python="&gt;=3.8.0">torch-2.0.1-cp38-cp38-win_amd64.whl</a><br/>
<a href="../../packages/88/6b/98ae5f6094b8ab12e00304f8474635b6f75a357764b9a2a4126252bc3ac4/torch-2.0.1-cp38-none-macosx_10_9_x86_64.whl#sha256=1adb60d369f2650cac8e9a95b1d5758e25d526a34808f7448d0bd599e4ae9072" data-requires-python="&gt;=3.8.0">torch-2.0.1-cp38-none-macosx_10_9_x86_64.whl</a><br/>
<a href="../../packages/3c/44/d0d3e07e03a17dfafaa3affbc07430d13b60b774e6fa495ab43828ef894e/torch-2.0.1-cp38-none-macosx_11_0_arm64.whl#sha256=1bcffc16b89e296826b33b98db5166f990e3b72654a2b90673e817b16c50e32b" data-requires-python="&gt;=3.8.0">torch-2.0.1-cp38-none-macosx_11_0_arm64.whl</a><br/>
<a href="../../packages/e5/9a/ce0fe125f226ffce8deba6a18bd8d0b9f589aa236780a83a6d70b5525f56/torch-2.0.1-cp39-cp39-manylinux1_x86_64.whl#sha256=e10e1597f2175365285db1b24019eb6f04d53dcd626c735fc502f1e8b6be9875" data-requires-python="&gt;=3.8.0">torch-2.0.1-cp39-cp39-manylinux1_x86_64.whl</a><br/>
<a href="../../packages/79/bb/0e1239b542d12b82cfb4c7d74359c68f0d23b536d1a4ac941a10b848488f/torch-2.0.1-cp39-cp39-manylinux2014_aarch64.whl#sha256=423e0ae257b756bb45a4b49072046772d1ad0c592265c5080070e0767da4e490" data-requires-python="&gt;=3.8.0">torch-2.0.1-cp39-cp39-manylinux2014_aarch64.whl</a><br/>
<a href="../../packages/48/f4/d0b61525a3d3db78636f1937d1bc24cbb39abc57484a545b72b6ab35c114/torch-2.0.1-cp39-cp39-win_amd64.whl#sha256=8742bdc62946c93f75ff92da00e3803216c6cce9b132fbca69664ca38cfb3e18" data-requires-python="&gt;=3.8.0">torch-2.0.1-cp39-cp39-win_amd64.whl</a><br/>
<a href="../../packages/1a/32/222e49ed160fa7d0690a938c42428fecb60b4595101d8e88ea523f70e406/torch-2.0.1-cp39-none-macosx_10_9_x86_64.whl#sha256=c62df99352bd6ee5a5a8d1832452110435d178b5164de450831a3a8cc14dc680" data-requires-python="&gt;=3.8.0">torch-2.0.1-cp39-none-macosx_10_9_x86_64.whl</a><br/>
<a href="../../packages/3c/67/7e19ebc15430f7385baee359383744c03d3600b51def9b399d0b8686e892/torch-2.0.1-cp39-none-macosx_11_0_arm64.whl#sha256=671a2565e3f63b8fe8e42ae3e36ad249fe5e567435ea27b94edaa672a7d0c416" data-requires-python="&gt;=3.8.0">torch-2.0.1-cp39-none-macosx_11_0_arm64.whl</a><br/>
</body>
</html>
<!--SERIAL 18020084-->
"#;
