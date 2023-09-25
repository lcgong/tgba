use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Hash, Clone, Debug, Serialize, Deserialize)]
pub enum PreRelease {
    Alpha(Option<u32>),
    Beta(Option<u32>),
    Preview(Option<u32>),
    ReleaseCandidate(Option<u32>),
}

impl PartialEq<Self> for PreRelease {
    fn eq(&self, other: &Self) -> bool {
        use PreRelease::{Alpha, Beta, Preview, ReleaseCandidate};
        match (self, other) {
            (Alpha(self_num), Alpha(other_num))
            | (Beta(self_num), Beta(other_num))
            | (ReleaseCandidate(self_num), ReleaseCandidate(other_num))
            | (Preview(self_num), Preview(other_num)) => {
                match (self_num, other_num) {
                    (None, None) => true,
                    (None, Some(_)) => false,
                    (Some(_), None) => false,
                    (Some(self_num), Some(other_num)) => {
                        //
                        self_num.eq(other_num)
                    }
                }
            }
            _ => false,
        }
    }
}

impl Eq for PreRelease {}

impl PartialOrd for PreRelease {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use PreRelease::{Alpha, Beta, Preview, ReleaseCandidate};

        match (self, other) {
            (Alpha(_), Preview(_))
            | (Beta(_), Preview(_))
            | (ReleaseCandidate(_), Preview(_))
            | (Preview(_), Alpha(_))
            | (Preview(_), Beta(_))
            | (Preview(_), ReleaseCandidate(_)) => None,
            (Alpha(_), Beta(_))
            | (Alpha(_), ReleaseCandidate(_))
            | (Beta(_), ReleaseCandidate(_)) => Some(Ordering::Less),
            (Beta(_), Alpha(_))
            | (ReleaseCandidate(_), Alpha(_))
            | (ReleaseCandidate(_), Beta(_)) => Some(Ordering::Greater),

            (Alpha(self_num), Alpha(other_num))
            | (Beta(self_num), Beta(other_num))
            | (ReleaseCandidate(self_num), ReleaseCandidate(other_num))
            | (Preview(self_num), Preview(other_num)) => {
                //
                match (self_num, other_num) {
                    (None, None) => Some(Ordering::Equal),
                    (None, Some(_)) => Some(Ordering::Less),
                    (Some(_), None) => Some(Ordering::Greater),
                    (Some(self_num), Some(other_num)) => {
                        //
                        self_num.partial_cmp(other_num)
                    }
                }
            }
        }
    }
}

//----------------------------------------------------------------------------
#[derive(Hash, Clone, Debug, Serialize, Deserialize)]
pub struct PostRelease {
    pub tag: Option<PostReleaseTag>,
    pub num: Option<u32>,
}

#[derive(Hash, Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum PostReleaseTag {
    Post,
    Rev,
}

impl PartialOrd for PostReleaseTag {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}

impl PartialEq<Self> for PostRelease {
    fn eq(&self, other: &Self) -> bool {
        match (self.num, other.num) {
            (None, None) => true,
            (None, Some(_)) => false,
            (Some(_), None) => false,
            (Some(self_num), Some(other_num)) => {
                //
                self_num.eq(&other_num)
            }
        }
    }
}

impl Eq for PostRelease {}

impl PartialOrd<Self> for PostRelease {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.num, other.num) {
            (None, None) => Some(Ordering::Equal),
            (None, Some(_)) => Some(Ordering::Less),
            (Some(_), None) => Some(Ordering::Greater),
            (Some(self_num), Some(other_num)) => {
                //
                self_num.partial_cmp(&other_num)
            }
        }
    }
}

//----------------------------------------------------------------------------
#[derive(Hash, Clone, Debug, Serialize, Deserialize)]
pub struct DevRelease {
    pub num: Option<u32>,
}

impl PartialEq<Self> for DevRelease {
    fn eq(&self, other: &Self) -> bool {
        match (self.num, other.num) {
            (None, None) => true,
            (None, Some(_)) => false,
            (Some(_), None) => false,
            (Some(self_num), Some(other_num)) => {
                //
                self_num.eq(&other_num)
            }
        }
    }
}

impl Eq for DevRelease {}

impl PartialOrd<Self> for DevRelease {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.num, other.num) {
            (None, None) => Some(Ordering::Equal),
            (None, Some(_)) => Some(Ordering::Less),
            (Some(_), None) => Some(Ordering::Greater),
            (Some(self_num), Some(other_num)) => {
                //
                self_num.partial_cmp(&other_num)
            }
        }
    }
}
