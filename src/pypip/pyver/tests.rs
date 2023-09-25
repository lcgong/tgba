use super::version::Version;

#[test]
fn test_pep440_release() {
    let versions = vec![
        "1",
        "1.2",
        "1.2.3",
        "1.2.3.4",
        "v1.1",
        "2.0",
        "2013.10",
        "2014.04",
        "1!1.0",
        "1!1.1",
        "1!2.0",
        "2!1.0.pre0",
        "1.0.dev456",
        "1.0a1",
        "1.0a2.dev456",
        "1.0a12.dev456",
        "1.0a12",
        "1.0b1.dev456",
        "1.0b2",
        "1.0b2.post345.dev456",
        "1.0b2.post345",
        "1.0rc1.dev456",
        "1.0rc1",
        "1.0",
        "1.0+abc.5",
        "1.0+abc.7",
        "1.0+5",
        "1.0.post456.dev34",
        "1.0.post456",
        "1.0.15",
        "1.1.dev1",
    ];

    for version in versions {
        match Version::parse(version) {
            Ok(v) => {
                println!("{:?}", v);
                continue;
            }
            Err(e) => panic!("Oh no {}", e),
        }
    }
}

#[test]
fn test_version_cmp() {
    use std::cmp::Ordering::{Equal, Greater, Less};

    assert_eq!(cmp("1", "1.0"), Some(Equal));
    assert_eq!(cmp("1", "1.0.0"), Some(Equal));
    assert_eq!(cmp("1.0a1", "1.0a1.dev1"), Some(Greater));
    assert_eq!(cmp("1.0a1", "1.0b1"), Some(Less));
    assert_eq!(cmp("1.0a1", "1.0"), Some(Less));
    assert_eq!(cmp("1.0rc1", "1.0"), Some(Less));
}

#[inline]
fn cmp(version1: &str, version2: &str) -> Option<std::cmp::Ordering> {
    let v1 = Version::parse(version1).unwrap();
    let v2 = Version::parse(version2).unwrap();

    v1.partial_cmp(&v2)
}