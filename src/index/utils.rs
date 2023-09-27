
use regex::Regex;
use lazy_static::lazy_static;

pub fn canonicalize_name(name: &str) -> String {
    lazy_static! {
        static ref CANONICALIZE_REGEX: Regex = Regex::new("[-_.]+").unwrap();
    }

    CANONICALIZE_REGEX.replace_all(name, "-").to_lowercase()
}