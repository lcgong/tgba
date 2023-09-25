use super::{DevRelease, PostRelease, PreRelease};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct Version {
    pub(super) original: String,
    pub(super) epoch: Option<u32>,
    pub(super) major: u32,
    pub(super) minor: u32,
    pub(super) patch: Option<u32>,
    pub(super) pre: Option<PreRelease>,
    pub(super) post: Option<PostRelease>,
    pub(super) dev: Option<DevRelease>,
    pub(super) local: Option<String>,
}

impl Version {
    pub fn epoch(&self) -> Option<u32> {
        self.epoch
    }

    pub fn major(&self) -> u32 {
        self.major
    }

    pub fn minor(&self) -> u32 {
        self.minor
    }

    pub fn patch(&self) -> Option<u32> {
        self.patch
    }

    pub fn is_prerelease(&self) -> bool {
        self.pre.is_some()
    }

    pub fn is_postrelease(&self) -> bool {
        self.post.is_some()
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.original)
    }
}

impl PartialEq<Self> for Version {
    fn eq(&self, other: &Self) -> bool {
        self.major == other.major
            && self.minor == other.minor
            && self.patch == other.patch
            && self.pre == other.pre
            && self.post == other.post
            && self.dev == other.dev
            && self.local == other.local
            && self.epoch == other.epoch
    }
}

impl Eq for Version {}

impl Hash for Version {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.major.hash(state);
        self.minor.hash(state);
        self.patch.hash(state);
        self.pre.hash(state);
        self.post.hash(state);
        self.dev.hash(state);
        self.local.hash(state);
        self.epoch.hash(state);
    }
}

impl PartialOrd for Version {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.epoch, other.epoch) {
            (None, None) => {}
            (None, Some(other_epoch)) => {
                if other_epoch != 0 {
                    return Some(Ordering::Less);
                }
            }
            (Some(self_epoch), None) => {
                if self_epoch != 0 {
                    return Some(Ordering::Greater);
                }
            }
            (Some(self_epoch), Some(other_epoch)) => {
                match self_epoch.partial_cmp(&other_epoch) {
                    Some(Ordering::Equal) => {}
                    v => {
                        return v;
                    }
                };
            }
        };

        if self.major < other.major {
            return Some(Ordering::Less);
        } else if self.major > other.major {
            return Some(Ordering::Greater);
        }

        if self.minor < other.minor {
            return Some(Ordering::Less);
        } else if self.minor > other.minor {
            return Some(Ordering::Greater);
        }

        match (self.patch, other.patch) {
            (None, None) => {}
            (None, Some(other_patch)) => {
                if other_patch != 0 {
                    return Some(Ordering::Less);
                }
            }
            (Some(self_patch), None) => {
                if self_patch != 0 {
                    return Some(Ordering::Greater);
                }
            }
            (Some(self_patch), Some(other_patch)) => {
                match self_patch.partial_cmp(&other_patch) {
                    Some(Ordering::Equal) => {}
                    v => {
                        return v;
                    }
                };
            }
        };

        match (&self.pre, &other.pre) {
            (None, None) => {}
            (None, Some(_)) => {
                return Some(Ordering::Greater);
            }
            (Some(_), None) => {
                return Some(Ordering::Less);
            }
            (Some(self_pre), Some(other_pre)) => {
                match self_pre.partial_cmp(other_pre) {
                    Some(Ordering::Equal) => {}
                    v => {
                        return v;
                    }
                };
            }
        };

        match (&self.post, &other.post) {
            (None, None) => {}
            (None, Some(_)) => {
                return Some(Ordering::Less);
            }
            (Some(_), None) => {
                return Some(Ordering::Greater);
            }
            (Some(self_post), Some(other_post)) => {
                match self_post.partial_cmp(other_post) {
                    Some(Ordering::Equal) => {}
                    v => {
                        return v;
                    }
                };
                //
            }
        };

        match (&self.dev, &other.dev) {
            (None, None) => {}
            (None, Some(_)) => {
                return Some(Ordering::Greater);
            }
            (Some(_), None) => {
                return Some(Ordering::Less);
            }
            (Some(self_dev), Some(other_dev)) => {
                match self_dev.partial_cmp(other_dev) {
                    Some(Ordering::Equal) => {}
                    v => {
                        return v;
                    }
                };
            }
        };

        Some(Ordering::Equal)
    }
}
