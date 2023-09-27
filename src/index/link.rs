use anyhow::{bail, Error};
use lazy_static::lazy_static;
use regex::{Regex, RegexBuilder};
use url::Url;

use super::utils::canonicalize_name;

#[derive(Debug)]
pub struct PackageLink {
    url: Url,
    requires_python: Option<String>,
    yanked_reason: Option<String>,
    hash: Option<(String, String)>, // (hash_method, hash_code)
    file_base: String,
    file_ext: String,
    pkg_version: String,
    wheel_info: Option<WheelInfo>,
}

impl PackageLink {
    pub fn requires_python(&self) -> Option<&str> {
        self.requires_python.as_deref()
    }

    pub fn file_base(&self) -> &str {
        &self.file_base
    }

    pub fn file_ext(&self) -> &str {
        &self.file_ext
    }

    pub fn wheel_info(&self) -> Option<&WheelInfo> {
        self.wheel_info.as_ref()
    }

    pub fn package_version(&self) -> &str {
        &self.pkg_version
    }
}

#[derive(Debug)]
pub struct WheelInfo {
    pyversions: Vec<String>,
    abis: Vec<String>,
    plats: Vec<String>,
    build: Option<String>,
}

impl WheelInfo {
    pub fn tags(&self) -> Vec<String> {
        let mut result = Vec::new();
        for ver in &self.pyversions {
            for abi in &self.abis {
                for plat in &self.plats {
                    result.push(format!("{}-{}-{}", ver, abi, plat));
                }
            }
        }

        result
    }
}

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

    // let pkg_name =  caps["name"].replace('_', "-");
    let pkg_version = caps["ver"].replace('_', "-");

    let pyversions = caps["pyver"].split('.').map(|s| s.to_string()).collect();
    let abis = caps["abi"].split('.').map(|s| s.to_string()).collect();
    let plats = caps["plat"].split('.').map(|s| s.to_string()).collect();
    let build = caps.name("build").map(|m| m.as_str().to_string());

    Ok((
        pkg_version,
        WheelInfo {
            pyversions,
            abis,
            plats,
            build,
        },
    ))
}

pub fn parse_link_from_url(
    canonical_name: &str,
    mut url: Url,
    requires_python: Option<&str>,
    yanked_reason: Option<&str>,
) -> Result<PackageLink, Error> {
    let hash = parse_link_hash(url.fragment());
    url.set_fragment(None);

    let requires_python = requires_python.map(|s| s.to_string());
    let yanked_reason = yanked_reason.map(|s| s.to_string());

    let (file_base, file_ext, prj_ver, wheel_info) =
        parse_url_file_name(url.as_str(), &canonical_name)?;

    Ok(PackageLink {
        url,
        requires_python,
        yanked_reason,
        hash,
        file_base,
        file_ext,
        pkg_version: prj_ver,
        wheel_info,
    })
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
