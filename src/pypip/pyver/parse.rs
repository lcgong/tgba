use lazy_static::lazy_static;
use pomsky_macro::pomsky;
use regex::Captures;
// use std::cmp::Ordering;
use super::{DevRelease, PostRelease, PostReleaseTag, PreRelease, Version};
use anyhow::{bail, Error};

/// https://github.com/Rust-Python-Packaging/pyver/blob/master/src/validator.rs
/// Rulex version of
/// Python's PEP-440 Regex
/// (<https://peps.python.org/pep-0440/#appendix-b-parsing-version-strings-with-regular-expressions>)
static VALIDATION_REGEX: &str = pomsky!(
    // Version String may start with v<version_number>
    // Example:
    // v1.0
    "v"?

    // Version String may include an epoch <epoch_num>!<version>
    // Example:
    // 1!1.0
    (:epoch(['0'-'9']+)'!')?

    // Version String must include major and minor version <major>.<minor>
    // Example:
    // 1.0
    :release(['0'-'9']+("."['0'-'9']+)*)

    // Version String may include Pre-Header
    // Example:
    // 1.0.preview-2
    // 1.0.rc2
    // 1.0beta2
    :pre(
        ["-" "_" "."]?

        :pretag(
        ("preview"|"alpha"|"beta"|"pre"|"rc"|"a"|"b"|"c")
        )

        ["-" "_" "."]?

        :prenum(['0'-'9']+)?
    )?

    // Version String may include Post-Header
    // Examples:
    // 1.0-9
    // 1.0-post2
    // 1.0.post.2
    :post(
        "-"
        :postnum1(['0'-'9']+)

        |

        ["-" "_" "."]?
        :posttag("post" | "rev" | "r")
        ["-" "_" "."]?
        :postnum2(['0'-'9']+)?
    )?

    // Version string may include Dev-header
    // Example:
    // 1.0-dev3
    // 1.0dev4
    // 1.0_dev_9
    :dev(
        ["-" "_" "."]?
        :devtag("dev")
        ["-" "_" "."]?
        :devnum(['0'-'9']+)?
    )?

    // Version string may include Local Version
    // Local version must start with +
    // Example:
    // 1.0+this.can.say.anything.as.long.as.its.a.letter.or.number.231241
    (
    "+"
    :local(
        ['a'-'z' '0'-'9']+
        ((["-" "_" "."] ['a'-'z' '0'-'9']+)+)?
    )
    )?
);

impl Version {
    pub fn parse(version: &str) -> Result<Version, Error> {
        let matched = match_version(version)?;

        let epoch = match matched.name("epoch") {
            Some(v) => Some(v.as_str().parse::<u32>()?),
            None => None,
        };

        let (major, minor, patch) = match matched.name("release") {
            Some(release) => {
                let splits: Vec<&str> = release.as_str().split('.').into_iter().collect();

                let n_splits = splits.len();

                let major = if n_splits >= 1 {
                    splits[0].parse::<u32>()?
                } else {
                    0
                };

                let minor = if n_splits >= 2 {
                    splits[1].parse::<u32>()?
                } else {
                    0
                };

                let patch = if n_splits >= 3 {
                    Some(splits[2].parse::<u32>()?)
                } else {
                    None
                };

                (major, minor, patch)
            }
            None => bail!("Failed to decode version {}", version),
        };

        let pre: Option<PreRelease> = match matched.name("pre") {
            Some(_) => {
                let pre_num = match matched.name("prenum") {
                    Some(v) => Some(v.as_str().parse::<u32>()?),
                    None => None,
                };

                match matched.name("pretag").unwrap().as_str() {
                    "alpha" => Some(PreRelease::Alpha(pre_num)),
                    "a" => Some(PreRelease::Alpha(pre_num)),
                    "beta" => Some(PreRelease::Beta(pre_num)),
                    "b" => Some(PreRelease::Beta(pre_num)),
                    "rc" => Some(PreRelease::ReleaseCandidate(pre_num)),
                    "c" => Some(PreRelease::ReleaseCandidate(pre_num)),
                    "preview" => Some(PreRelease::Preview(pre_num)),
                    "pre" => Some(PreRelease::Preview(pre_num)),
                    _ => None,
                }
            }
            None => None,
        };

        let post = match matched.name("post") {
            Some(_) => {
                let post_num: Option<u32> = match matched.name("postnum1") {
                    Some(v) => Some(v.as_str().parse::<u32>()?),
                    None => match matched.name("postnum2") {
                        Some(v) => Some(v.as_str().parse::<u32>()?),
                        _ => None,
                    },
                };

                let post_head: Option<PostReleaseTag> = match matched.name("posttag") {
                    Some(v) => match v.as_str() {
                        "post" => Some(PostReleaseTag::Post),
                        "rev" => Some(PostReleaseTag::Rev),
                        "r" => Some(PostReleaseTag::Rev),
                        _ => None,
                    },
                    None => None,
                };

                Some(PostRelease {
                    tag: post_head,
                    num: post_num,
                })
            }
            None => None,
        };

        let dev = match matched.name("dev") {
            Some(_) => {
                let dev_num = match matched.name("devnum") {
                    Some(v) => Some(v.as_str().parse::<u32>()?),
                    None => None,
                };
                Some(DevRelease { num: dev_num })
            }
            None => None,
        };

        let local = matched.name("local").map(|v| v.as_str().to_string());

        Ok(Version {
            original: version.to_string(),
            epoch,
            major,
            minor,
            patch,
            pre,
            post,
            dev,
            local,
        })
    }
}

fn match_version(version: &str) -> Result<Captures, Error> {
    lazy_static! {
        static ref VERSION_PARSE: regex::Regex = regex::Regex::new(VALIDATION_REGEX).unwrap();
    }

    match VERSION_PARSE.captures(version) {
        Some(groups) => Ok(groups),
        None => anyhow::bail!("Failed to decode version {}", version),
    }
}
