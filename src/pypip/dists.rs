use super::version::PythonVersion;
use std::borrow::Cow;

pub const PYTHON_VERSIONS: &[(PythonVersion, &str, &str, &str, Option<&str>)] = &[
    // (PythonVersion { kind: Cow::Borrowed("cpython"), major: 3, minor: 11, patch: 5, suffix: None }, "x86_64", "linux", "https://github.com/indygreg/python-build-standalone/releases/download/20230826/cpython-3.11.5%2B20230826-x86_64-unknown-linux-gnu-pgo%2Blto-full.tar.zst", Some("556d7d46c2af6f9744da03cac5304975f60de1cd5846a109814dd5c396fe9042")),
    // (PythonVersion { kind: Cow::Borrowed("cpython"), major: 3, minor: 11, patch: 5, suffix: None }, "x86_64", "macos", "https://github.com/indygreg/python-build-standalone/releases/download/20230826/cpython-3.11.5%2B20230826-x86_64-apple-darwin-pgo%2Blto-full.tar.zst", Some("e43d70a49919641ca2939a5a9107b13d5fef8c13af0f511a33a94bb6af2044f0")),
    (PythonVersion { kind: Cow::Borrowed("cpython"), major: 3, minor: 11, patch: 5, suffix: None }, 
        "x86_64", "windows", 
        "https://gitee.com/lyucg/python-dists/releases/download/20230826/cpython-3.11.5-20230826-i686-pc-windows-msvc-shared-pgo-full.tar.zst",
        // "https://github.com/indygreg/python-build-standalone/releases/download/20230826/cpython-3.11.5%2B20230826-x86_64-pc-windows-msvc-shared-pgo-full.tar.zst", 
        Some("c9ffe9c2c88685ce3064f734cbdfede0a07de7d826fada58f8045f3bd8f81a9d")
    ),
    (PythonVersion { kind: Cow::Borrowed("cpython"), major: 3, minor: 8, patch: 17, suffix: None }, 
        "x86_64", "windows", 
        "https://gitee.com/lyucg/python-dists/releases/download/20230826/cpython-3.8.17-20230826-i686-pc-windows-msvc-shared-pgo-full.tar.zst",
        // "https://github.com/indygreg/python-build-standalone/releases/download/20230826/cpython-3.8.17%2B20230826-x86_64-pc-windows-msvc-shared-pgo-full.tar.zst", 
        Some("0931d8ca0e060c6ac1dfcf6bb9b6dea0ac3a9d95daf7906a88128045f4464bf8")
    ),
    // (PythonVersion { kind: Cow::Borrowed("cpython"), major: 3, minor: 8, patch: 10, suffix: None }, "x86_64", "linux", "https://github.com/indygreg/python-build-standalone/releases/download/20210506/cpython-3.8.10-x86_64-unknown-linux-gnu-pgo%2Blto-20210506T0943.tar.zst", None),
    // (PythonVersion { kind: Cow::Borrowed("cpython"), major: 3, minor: 8, patch: 10, suffix: None }, "x86_64", "macos", "https://github.com/indygreg/python-build-standalone/releases/download/20210506/cpython-3.8.10-x86_64-apple-darwin-pgo%2Blto-20210506T0943.tar.zst", None),
    // (PythonVersion { kind: Cow::Borrowed("cpython"), major: 3, minor: 8, patch: 10, suffix: None }, "x86_64", "windows", "https://github.com/indygreg/python-build-standalone/releases/download/20210506/cpython-3.8.10-x86_64-pc-windows-msvc-shared-pgo-20210506T0943.tar.zst", None),
];
