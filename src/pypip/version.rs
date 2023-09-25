use anyhow::{anyhow, bail, Error};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
// pub const DEFAULT_KIND: &str = "cpython";
// pub const SELF_PYTHON_TARGET_VERSION: PythonVersion = PythonVersion {
//     kind: Cow::Borrowed("cpython"),
//     major: 3,
//     minor: 11,
//     patch: 5,
//     suffix: None,
// };

/// Internal descriptor for a python version request.
// #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone)]
// pub struct PythonVersionRequest {
//     pub kind: Option<Cow<'static, str>>,
//     pub major: u8,
//     pub minor: Option<u8>,
//     pub patch: Option<u8>,
//     pub suffix: Option<Cow<'static, str>>,
// }

/// Internal descriptor for a python version.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone)]
pub struct PythonVersion {
    pub kind: Cow<'static, str>,
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
    pub suffix: Option<Cow<'static, str>>,
}

// impl TryFrom<PythonVersionRequest> for PythonVersion {
//     type Error = Error;

//     fn try_from(req: PythonVersionRequest) -> Result<Self, Self::Error> {
//         Ok(PythonVersion {
//             kind: match req.kind {
//                 None => Cow::Borrowed(DEFAULT_KIND),
//                 Some(other) => other,
//             },
//             major: req.major,
//             minor: req.minor.ok_or_else(|| anyhow!("missing minor version"))?,
//             patch: req.patch.ok_or_else(|| anyhow!("missing patch version"))?,
//             suffix: req.suffix,
//         })
//     }
// }

impl std::fmt::Display for PythonVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}@{}.{}.{}",
            self.kind, self.major, self.minor, self.patch
        )?;
        if let Some(ref suffix) = self.suffix {
            write!(f, ".{}", suffix)?;
        }
        Ok(())
    }
}

// impl std::fmt::Display for PythonVersionRequest {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         if let Some(ref kind) = self.kind {
//             write!(f, "{}@", kind)?;
//         }
//         write!(f, "{}", self.major)?;
//         if let Some(ref minor) = self.minor {
//             write!(f, ".{}", minor)?;
//             if let Some(ref patch) = self.patch {
//                 write!(f, ".{}", patch)?;
//             }
//         }
//         Ok(())
//     }
// }





