use anyhow::{bail, Error};
use lazy_static::lazy_static;
use regex::{Regex, RegexBuilder};
use url::Url;

use super::super::utils::{canonicalize_name, split_filename_extension};


#[derive(Debug)]
pub struct PackageLink {
    url: Url,
    requires_python: Option<String>,
    yanked_reason: Option<String>,
    hash: Option<(String, String)>, // (hash_method, hash_code)
    filename_base: String,
    filename_extension: String,
    package_version: String,
    wheel_info: Option<WheelInfo>,
}

impl PackageLink {
    pub fn requires_python(&self) -> Option<&str> {
        self.requires_python.as_deref()
    }

    pub fn filename_base(&self) -> &str {
        &self.filename_base
    }

    pub fn filename_extension(&self) -> &str {
        &self.filename_extension
    }

    // pub fn wheel_info(&self) -> Option<&WheelInfo> {
    //     self.wheel_info.as_ref()
    // }

    pub fn package_version(&self) -> &str {
        &self.package_version
    }

    pub fn is_wheel(&self) -> bool {
        self.wheel_info.is_some()
    }

    pub fn wheel_tags(&self) -> Option<Vec<String>> {
        let Some(wheel) = &self.wheel_info else {
            return None;
        };

        let mut tags = Vec::new();
        for ver in &wheel.pyversions {
            for abi in &wheel.abis {
                for plat in &wheel.plats {
                    tags.push(format!("{}-{}-{}", ver, abi, plat));
                }
            }
        }

        Some(tags)

    }
}

#[derive(Debug)]
struct WheelInfo {
    pyversions: Vec<String>,
    abis: Vec<String>,
    plats: Vec<String>,
    build: Option<String>,
}

// impl WheelInfo {
//     pub fn tags(&self) -> Vec<String> {
//         let mut result = Vec::new();
//         for ver in &self.pyversions {
//             for abi in &self.abis {
//                 for plat in &self.plats {
//                     result.push(format!("{}-{}-{}", ver, abi, plat));
//                 }
//             }
//         }

//         result
//     }
// }

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
        filename_base: file_base,
        filename_extension: file_ext,
        package_version: prj_ver,
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

/// 从打包的文件名拆分出版本信息。
/// 例如：pkg_name-1.2.3.tar.gz，pkg-name-1.2.3.tar.gz
fn split_version_from_filename(filename: &str, canonical_name: &str) -> Option<usize> {
    for (i, ch) in filename.chars().enumerate() {
        if ch != '-' {
            continue;
        }

        if canonicalize_name(&filename[..i]) == canonical_name {
            return Some(i + 1);
        }
    }
    None
}

fn parse_url_file_name(
    url: &str,
    canonical_name: &str,
) -> Result<(String, String, String, Option<WheelInfo>), Error> {
    // 从url拆分出文件名
    let splits = url.rsplit_once('/').unwrap();
    let file_name = splits.1;

    // 从文件名拆分出文件名后缀
    let (filename_base, filename_ext) = split_filename_extension(&file_name)?;

    let (package_version, wheel) = if is_wheel_file(filename_ext) {
        let (package_version, wheel) = parse_wheel_info(filename_base)?;

        (package_version, Some(wheel))
    } else {
        if !is_archive_extension(filename_ext) {
            bail!("no support extension: {}", file_name);
        }

        let Some(version_start) = split_version_from_filename(filename_base, canonical_name) else {
            panic!("{} does not match {}", filename_base, canonical_name)
        };

        let package_version = filename_base[version_start..].to_string();

        (package_version, None)
    };

    Ok((
        filename_base.to_string(),
        filename_ext.to_string(),
        package_version,
        wheel,
    ))
}



lazy_static! {
    static ref WHEEL_EXTENSION: &'static str = ".whl";
}

#[inline]
fn is_wheel_file(filename_ext: &str) -> bool {
    filename_ext.to_lowercase() == *WHEEL_EXTENSION
}

#[inline]
fn is_archive_extension(extension: &str) -> bool {
    let extension = extension.to_lowercase();
    lazy_static! {
        static ref SUPPORT_EXTENSIONS: [&'static str; 11]  = [
            ".tar.gz", ".tgz", ".tar", // tar
            ".zip", // zip
            ".tar.bz2", ".tbz", // bz2
            ".tar.xz", ".txz", ".tlz", ".tar.lz", ".tar.lzma", // xz
        ];
    };

    for ext in SUPPORT_EXTENSIONS.into_iter() {
        if extension == ext {
            return true;
        }
    }

    false
}


