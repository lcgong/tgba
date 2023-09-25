///
/// 
/// https://peps.python.org/pep-0440
mod parse;
mod phases;
mod version;

pub use phases::{DevRelease, PostRelease, PostReleaseTag, PreRelease};
pub use version::Version;

#[cfg(test)]
mod tests;
