use anyhow::{anyhow, bail, Error, Ok};
use scraper::{Html, Selector};
use url::Url;

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

#[derive(Debug)]
pub struct Link {
    url: Url,
    requires_python: Option<String>,
    yanked_reason: Option<String>,
    hash: Option<(String, String)>, // (hash_method, hash_code)
    file_base: String,
    file_ext: String,
    prj_ver: String,
    wheel_info: Option<WheelInfo>,
}

pub async fn get_project_index(client: &reqwest::Client) -> Result<(), Error> {
    let project_name = "jupyterlab";
    let canonical_name = canonicalize_name(project_name);

    let project_url = "https://pypi.tuna.tsinghua.edu.cn/simple/jupyterlab/";
    // let resp = client.get(url).send().await?;
    // let page = resp.text().await?;

    let document = Html::parse_document(PAGE_CONTENT);
    let link_selector = Selector::parse("body > a").unwrap();
    let base_selector = Selector::parse("base").unwrap();

    let base_url = Url::parse(match document.select(&base_selector).next() {
        Some(node) => {
            //
            node.value().attr("href").unwrap_or(project_url)
        }
        None => project_url,
    })?;

    println!("base: {}", base_url);

    for node in document.select(&link_selector) {
        let elem = node.value();
        let href = match elem.attr("href") {
            Some(href) => href,
            None => continue,
        };

        let mut url = base_url.join(href)?;
        let requires_python = elem.attr("data-requires-python").map(|s| s.to_string());
        let yanked_reason = elem.attr("data-yanked").map(|s| s.to_string());
        let hash = parse_link_hash(url.fragment());

        url.set_fragment(None);

        let (file_base, file_ext, prj_ver, wheel_info) =
            parse_url_file_name(url.as_str(), &canonical_name)?;

        let link = Link {
            url,
            requires_python,
            yanked_reason,
            hash,
            file_base,
            file_ext,
            prj_ver,
            wheel_info,
        };

        println!("{:?}\n", link);
    }
    Ok(())
}

impl Link {
    pub fn requires_python(&self) -> Option<&str> {
        self.requires_python.as_deref()
    }

    pub fn ext(&self) -> &str {
        &self.file_ext
    }

    pub fn filename(&self) -> &str {
        let splits = self.url.as_str().rsplit_once('/').unwrap();
        splits.1
    }
}

#[derive(Debug)]
pub struct WheelInfo {
    pyver: String,
    abi: String,
    plat: String,
    build: Option<String>,
}

use lazy_static::lazy_static;

fn parse_wheel_info(file_base: &str) -> Result<(String, WheelInfo), Error> {
    lazy_static! {
        /// https://github.com/pypa/pip/blob/main/src/pip/_internal/models/wheel.py
        static ref WHEEL_INFO_REGEX: Regex = RegexBuilder::new(
            r#"^(?P<namever>(?P<name>[^\s-]+?)-(?P<ver>[^\s-]*?))
            (
                (-(?P<build>\d[^-]*?))?
                -(?P<pyver>[^\s-]+?)
                -(?P<abi>[^\s-]+?)
                -(?P<plat>[^\s-]+?)
            )$"#
        )
        .ignore_whitespace(true)
        .build()
        .unwrap();
    }

    let Some(caps) = WHEEL_INFO_REGEX.captures(file_base) else {
        bail!("error in parsing: {}", file_base)
    };

    Ok((
        caps["ver"].to_string(),
        WheelInfo {
            pyver: caps["pyver"].to_string(),
            abi: caps["abi"].to_string(),
            plat: caps["plat"].to_string(),
            build: caps.name("build").map(|m| m.as_str().to_string()),
        },
    ))
}


fn parse_link_hash(url_fragment: Option<&str>) -> Option<(String, String)> {
    let Some(url_fragment) = url_fragment else {
        return None;
    };

    lazy_static! {
        static ref HASH_REGEX: regex::Regex =
            regex::Regex::new("[#&]?(sha512|sha384|sha256|sha224|sha1|md5)=([^&]*)").unwrap();
    }

    let Some(caps) = HASH_REGEX.captures(url_fragment) else {
        return None;
    };

    Some((caps[1].to_string(), (caps[2].to_string())))
}

use regex::{Regex, RegexBuilder};

pub fn canonicalize_name(name: &str) -> String {
    lazy_static! {
        static ref CANONICALIZE_REGEX: Regex = Regex::new("[-_.]+").unwrap();
    }

    CANONICALIZE_REGEX.replace_all(name, "-").to_string()
}

fn split_name_version(filename: &str, canonical_name: &str) -> Option<usize> {
    for (i, ch) in filename.chars().enumerate() {
        if ch != '-' {
            continue;
        }

        if canonicalize_name(&filename[..i]) == canonical_name {
            return Some(i);
        }
    }
    None
}

fn parse_url_file_name(
    url: &str,
    canonical_name: &str,
) -> Result<(String, String, String, Option<WheelInfo>), Error> {
    let splits = url.rsplit_once('/').unwrap();
    let file_name = splits.1;

    let Some(mut sep) = file_name.rfind('.') else {
        bail!("invalid filename: {}", file_name);
    };

    if file_name[..sep].to_lowercase().ends_with(".tar") {
        sep -= 4;
    }

    let (file_base, file_ext) = (&file_name[..sep], &file_name[sep..]);

    let (prj_ver, wheel) = if file_ext == ".whl" {
        let (prj_ver, wheel) = parse_wheel_info(file_base)?;

        (prj_ver, Some(wheel))
    } else {
        let Some(sep) = split_name_version(file_base, canonical_name) else {
            panic!("{} does not match {}", file_base, canonical_name)
        };

        let prj_ver = file_base[(sep + 1)..].to_string();

        (prj_ver, None)
    };

    Ok((file_base.to_string(), file_ext.to_string(), prj_ver, wheel))
}
