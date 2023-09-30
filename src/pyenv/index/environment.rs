
use std::collections::HashMap;
// use super::link::WheelInfo;

// pub struct TargetEnv {
//     python_version: String,
//     support_tags_map: HashMap<String, u32>,
// }

// impl TargetEnv {
//     pub fn new() -> Self {
//         let mut support_tags_map = HashMap::new();
//         for (i, tag) in SUPPORT_TAGS.iter().enumerate() {
//             support_tags_map.insert(tag.to_string(), i as u32);
//         }

//         TargetEnv {
//             python_version: "3.11.5".to_string(),
//             support_tags_map,
//         }
//     }

//     pub fn python_version(&self) -> &str {
//         &self.python_version
//     }

//     pub fn get_best_tag_rank(&self, wheel: &WheelInfo) -> Option<u32> {
//         let mut ranks = Vec::new();
//         for tag in wheel.tags() {
//             if let Some(i) = self.support_tags_map.get(tag.as_str()) {
//                 ranks.push(*i);
//             }
//         }

//         ranks.iter().min().copied()
//     }
// }


static SUPPORT_TAGS: [&str; 39] = [
    "cp311-cp311-win_amd64",
    "cp311-abi3-win_amd64",
    "cp311-none-win_amd64",
    "cp310-abi3-win_amd64",
    "cp39-abi3-win_amd64",
    "cp38-abi3-win_amd64",
    "cp37-abi3-win_amd64",
    "cp36-abi3-win_amd64",
    "cp35-abi3-win_amd64",
    "cp34-abi3-win_amd64",
    "cp33-abi3-win_amd64",
    "cp32-abi3-win_amd64",
    "py311-none-win_amd64",
    "py3-none-win_amd64",
    "py310-none-win_amd64",
    "py39-none-win_amd64",
    "py38-none-win_amd64",
    "py37-none-win_amd64",
    "py36-none-win_amd64",
    "py35-none-win_amd64",
    "py34-none-win_amd64",
    "py33-none-win_amd64",
    "py32-none-win_amd64",
    "py31-none-win_amd64",
    "py30-none-win_amd64",
    "cp311-none-any",
    "py311-none-any",
    "py3-none-any",
    "py310-none-any",
    "py39-none-any",
    "py38-none-any",
    "py37-none-any",
    "py36-none-any",
    "py35-none-any",
    "py34-none-any",
    "py33-none-any",
    "py32-none-any",
    "py31-none-any",
    "py30-none-any",
];
