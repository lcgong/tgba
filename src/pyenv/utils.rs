use anyhow::{bail, Result};
use once_cell::sync::Lazy;
use pep440_rs::Version;
use regex::Regex;
use std::path::PathBuf;

pub fn parse_version(version: &str) -> Result<Version> {
    use std::str::FromStr;

    Ok(match Version::from_str(version) {
        Ok(version) => version,
        Err(err) => bail!("parsing version: {}", err),
    })
}

static CANONICALIZE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("[-_.]+").unwrap());

pub fn canonicalize_name(name: &str) -> String {
    CANONICALIZE_REGEX.replace_all(name, "-").to_lowercase()
}

pub fn make_python_bin_path(pydist_dir: &PathBuf) -> PathBuf {
    let mut pybin_path = pydist_dir.clone();
    // 支持目录结构 install/bin/python, install/python and bin/python
    pybin_path.push("install");
    if !pybin_path.is_dir() {
        pybin_path.pop();
    }
    pybin_path.push("bin");
    if !pybin_path.is_dir() {
        pybin_path.pop();
    }

    #[cfg(unix)]
    {
        pybin_path.push("python3");
    }
    #[cfg(windows)]
    {
        pybin_path.push("python.exe");
    }

    pybin_path
}

pub fn split_filename_extension(file_name: &str) -> Result<(&str, &str)> {
    let Some(mut sep) = file_name.rfind('.') else {
        bail!("no extension: {}", file_name);
    };

    if file_name[..sep].to_lowercase().ends_with(".tar") {
        sep -= 4;
    }

    let filename_base = &file_name[..sep];
    let filename_ext = &file_name[sep..];

    Ok((filename_base, filename_ext))
}

pub fn get_windows_major_versoin() -> Result<u8> {
    use winreg::enums::HKEY_LOCAL_MACHINE;
    use winreg::RegKey;

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

    let subkey = r"SOFTWARE\Microsoft\Windows NT\CurrentVersion";
    let Ok(key) = hklm.open_subkey(subkey) else {
        bail!("无法打开注册表{}", subkey)
    };

    // 获取ProductName值
    let property_name = "ProductName";
    let Ok(product_name): Result<String, _> = key.get_value(property_name) else {
        bail!("无法读取注册表{}的{}", subkey, property_name)
    };

    // 检查ProductName来确定操作系统版本
    if product_name.contains("Windows 7") {
        Ok(7)
    } else if product_name.contains("Windows 10") {
        Ok(10)
    } else if product_name.contains("Windows 8") {
        Ok(8)
    } else {
        bail!("不支持的Windows版本")
    }
}
