use anyhow::{bail, Result};
use std::fs::File;
use std::process::{Command, Stdio};

use super::archive::{checksum, unpack_archive};
use super::download::download;
use super::utils::make_python_bin_path;
use super::Installer;

pub async fn ensure_python_venv(installer: &mut Installer) -> Result<()> {
    ensure_python_dist(installer).await?;

    ensure_venv(installer).await?;

    set_platform_info(installer)?;

    Ok(())
}

static PLATFORM_INFO_SCRIPT: &str = r#"
import json
import sysconfig
from pip._internal.utils.compatibility_tags import get_supported

print(json.dumps({
    "platform_tag": sysconfig.get_platform().replace('.', '-').replace('-', '_'),
    "support_tags": [str(t) for t in get_supported()]
}))
"#;

pub fn set_platform_info(installer: &mut Installer) -> Result<()> {
    let tmp_dir = tempfile::tempdir()?;

    let script_file = tmp_dir.path().join("platform_info.py");
    let Ok(mut file) = File::create(&script_file) else {
        bail!("无法创建临时脚本文件: {}", script_file.display())
    };

    use std::io::Write;
    file.write_all(PLATFORM_INFO_SCRIPT.as_bytes())?;
    file.sync_all()?;

    println!("file: {}", &script_file.display());

    let output = Command::new(&installer.venv_python_path)
        .arg(&script_file)
        .stdout(Stdio::piped())
        .output()
        .expect("无法执行Python脚本");

    let output = String::from_utf8_lossy(&output.stdout);

    let json_msg: serde_json::Value = serde_json::from_str(&output)?;

    installer.platform_tag = Some(json_msg["platform_tag"].as_str().unwrap().to_string());
    let support_tags_map = &mut installer.support_tags_map;

    for (i, tag) in json_msg["support_tags"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap())
        .enumerate()
    {
        support_tags_map.insert(tag.to_string(), i as u32);
    }

    Ok(())
}

pub async fn ensure_python_dist(installer: &Installer) -> Result<()> {
    let pydist_dir = &installer.pydist_dir;
    let python_bin = make_python_bin_path(pydist_dir);

    let pyver = installer.python_version_full.as_str();

    if pydist_dir.is_dir() && python_bin.is_file() {
        installer.log(format!("自带CPython-{}已安装", pyver).as_str());
        return Ok(());
    }

    if let Err(_err) = std::fs::create_dir_all(pydist_dir) {
        bail!("创建目录{}失败: {}", pydist_dir.display(), _err)
    }

    let cpython_source = &installer.pydist_source;

    // let (dist_url, dist_digest) = &installer.pydist_source;

    use super::utils::split_filename_extension;

    let Ok((_file_base, file_ext)) = split_filename_extension(cpython_source.url()) else {
        bail!("地址文件解析扩展名错误: {}", cpython_source.url())
    };

    let buffer = download(
        installer,
        cpython_source.url(),
        &format!("下载CPython-{}安装包", pyver),
    )
    .await?;

    if !checksum("sha256", &buffer, cpython_source.checksum())? {
        bail!("hash check of {} failed", cpython_source.url())
    }

    installer.log(format!("解压CPython-{}安装包", pyver).as_str());
    unpack_archive(file_ext, &buffer, pydist_dir)?;

    installer.log(format!("CPython-{}安装完成", installer.python_version_full).as_str());

    Ok(())
}

pub async fn ensure_venv(installer: &Installer) -> Result<()> {
    let venv_dir = &installer.venv_dir;
    let python_bin = make_python_bin_path(&installer.pydist_dir);

    let flag_done = venv_dir.join(".TGBA_VENV_DONE");

    if venv_dir.is_dir() && flag_done.is_file() {
        installer.log("虚拟环境已经创建，跳过该任务");
        return Ok(());
    }

    installer.log("创建Python虚拟环境");

    // initialize the virtualenv
    let mut venv_cmd = Command::new(&python_bin);
    venv_cmd.arg("-mvenv");
    venv_cmd.arg(&venv_dir);

    let status = match venv_cmd.status() {
        Ok(status) => status,
        Err(_err) => {
            bail!("unable to create self venv using {}", python_bin.display())
        }
    };

    if !status.success() {
        bail!("failed to initialize virtualenv in {}", venv_dir.display());
    }

    let flag_done = venv_dir.join(".TGBA_VENV_DONE");
    let Ok(flag_done) = std::fs::File::create(flag_done) else {
        bail!("无法新建环境创建完成标记文件")
    };
    drop(flag_done);

    installer.log("完成创建Python虚拟环境");

    Ok(())
}

pub fn venv_python_cmd(installer: &Installer, args: &[&str]) -> Result<std::process::Output> {
    let python_bin = &installer.venv_python_path;

    // 将venv/Script目录添加到环境变量PATH中
    use std::env;
    let venv_script_dir = installer.venv_dir.join("Scripts");
    let path_env = if let Some(path) = env::var_os("PATH") {
        let mut paths = std::env::split_paths(&path).collect::<Vec<_>>();
        paths.insert(0, venv_script_dir);
        env::join_paths(paths)?
    } else {
        let paths = vec![venv_script_dir];
        env::join_paths(paths)?
    };

    let mut cmd = Command::new(&python_bin);
    cmd.env("PATH", path_env.to_string_lossy().as_ref());
    cmd.env("VIRTUAL_ENV", installer.venv_dir.to_string_lossy().as_ref());
    cmd.env_remove("PYTHONHOME");
    for arg in args {
        cmd.arg(arg);
    }

    let args_str = cmd
        .get_args()
        .map(|s| s.to_string_lossy().to_string())
        .collect::<Vec<String>>()
        .join(" ");
    let prog_cmd = format!("{} {}", cmd.get_program().to_string_lossy(), args_str);

    let output = match cmd.output() {
        Ok(output) => output,
        Err(err) => {
            use std::io::ErrorKind;
            if err.kind() == ErrorKind::Interrupted {
                bail!("程序({})异常中断: {}", prog_cmd, err)
            } else {
                bail!("程序({})无法执行：{}", prog_cmd, err)
            }
        }
    };

    Ok(output)
}

// pub fn venv_cmd(installer: &Installer) -> Result<()> {
//     // 将venv/Script目录添加到环境变量PATH中
//     use std::env;
//     let venv_script_dir = installer.venv_dir.join("Scripts");
//     let path_env = if let Some(path) = env::var_os("PATH") {
//         let mut paths = std::env::split_paths(&path).collect::<Vec<_>>();
//         paths.insert(0, venv_script_dir);
//         env::join_paths(paths)?
//     } else {
//         let paths = vec![venv_script_dir];
//         env::join_paths(paths)?
//     };

//     println!("path: {}", &path_env.to_string_lossy());

//     let mut cmd = Command::new("cmd.exe");
//     cmd.env("PATH", path_env.to_string_lossy().as_ref());
//     cmd.env("VIRTUAL_ENV", installer.venv_dir.to_string_lossy().as_ref());
//     cmd.env_remove("PYTHONHOME");

//     cmd.arg("/c");
//     cmd.arg("set");

//     let args_str = cmd
//         .get_args()
//         .map(|s| s.to_string_lossy().to_string())
//         .collect::<Vec<String>>()
//         .join(" ");
//     let prog_cmd = format!("{} {}", cmd.get_program().to_string_lossy(), args_str);
//     println!("cmd: {}", prog_cmd);

//     let output = match cmd.output() {
//         Ok(output) => output,
//         Err(err) => {
//             use std::io::ErrorKind;
//             if err.kind() == ErrorKind::Interrupted {
//                 bail!("程序({})异常中断: {}", prog_cmd, err)
//             } else {
//                 bail!("程序({})无法执行：{}", prog_cmd, err)
//             }
//         }
//     };

//     println!(
//         "STATUS:{}\n{}\nSTDERR:\n{}",
//         output.status,
//         String::from_utf8_lossy(&output.stdout),
//         String::from_utf8_lossy(&output.stderr)
//     );

//     Ok(())
// }
