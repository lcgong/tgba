use anyhow::{bail, Result};
use pep508_rs::Requirement;
use std::{fs::File, path::PathBuf};

use crate::errors::DownloadingError;

use super::super::status::StatusUpdate;
use super::installer::Installer;

pub async fn prepare_requirements(installer: &Installer) -> Result<Vec<Requirement>> {
    let cached_packages_dir = &installer.cached_packages_dir;
    if let Err(_err) = std::fs::create_dir_all(cached_packages_dir) {
        bail!(
            "创建下载文件临时目录{}失败: {}",
            cached_packages_dir.display(),
            _err
        )
    }

    let requirements_path = &get_requirements_path(installer).await?;

    log::info!("程序包需求文件: {}", requirements_path.display());
    log::info!("程序包下载临时目录: {}", cached_packages_dir.display());

    let mut requirements = extract_requirements(requirements_path).await?;
    requirements.append(&mut get_obligated_requirements(installer)?);

    Ok(requirements)
}

// pub async fn download_requirements(
//     installer: &Installer,
//     collector: &impl StatusUpdate,
// ) -> Result<()> {
//     let n_downloads = requirements.len();

//     for (idx, requirement) in requirements.iter().enumerate() {
//         retry_download_requirement(installer, collector, requirement).await?;

//         collector.update_progress(idx as u32 + 1, n_downloads as u32);
//     }

//     Ok(())
// }

pub async fn retry_download_requirement(
    installer: &Installer,
    collector: &impl StatusUpdate,
    requirement: &Requirement,
) -> Result<(), DownloadingError> {
    use super::project::download_requirement;

    let mut errors = Vec::new();
    for pypi in installer.pypi_mirrors() {
        match download_requirement(installer, collector, &pypi, requirement).await {
            Ok(_) => {
                return Ok(());
            }
            Err(err) => {
                log::error!(
                    "从{}下载{}中发生错误: {}",
                    pypi.name(),
                    requirement.name,
                    err
                );
                let pypi_name = pypi.name();
                errors.push(format!("尝试从{pypi_name}镜像下载发生错误: {err}"));
            }
        };
    }

    let details = errors.join("\n");
    let pkg_name = &requirement.name;

    return Err(DownloadingError::error(format!(
        "下载{pkg_name}发生错误:\n{details}"
    )));

    // let mirrors = installer
    //     .pypi_mirrors()
    //     .iter()
    //     .map(|m| m.name())
    //     .collect::<Vec<&str>>()
    //     .join(",");

    // bail!(
    //     "在现有PYPI镜像({})中没有发现需求{}所对应可用的程序包",
    //     mirrors,
    //     requirement.name
    // )
}

pub async fn offline_install_requirements(
    installer: &Installer,
) -> Result<()> {
    use super::venv::venv_python_cmd;

    let requirements_path = get_requirements_path(installer)
        .await?
        .to_string_lossy()
        .to_string();
    let cached_packages_dir = installer.cached_packages_dir.to_string_lossy().to_string();

    let args = &vec![
        "-m",
        "pip",
        "install",
        "--no-index",
        "--find-links",
        &cached_packages_dir,
        "-r",
        &requirements_path,
    ];

    log::info!(
        "开始从本地{}安装程序需求{}",
        cached_packages_dir,
        requirements_path,
    );

    let err = match venv_python_cmd(installer, args) {
        Ok(output) => {
            if output.status.success() {
                return Ok(());
            }

            String::from_utf8_lossy(&output.stderr).to_string()
        }
        Err(err) => err.to_string(),
    };

    bail!("程序包本地安装发生错误: {}", err)
}

use super::super::resources::RESOURCES;

async fn get_requirements_path(installer: &Installer) -> Result<PathBuf> {
    let filename = format!(
        "requirements-{}-{}.txt",
        installer.python_version,
        installer.platform_tag.as_ref().unwrap()
    );

    let requirements_path = installer.tgba_dir().join(&filename);

    let mut file = File::create(&requirements_path)?;

    use std::io::Write;
    file.write_all(RESOURCES.get_requirements_file(&installer.python_version))?;

    Ok(requirements_path)
}

async fn extract_requirements(requirements_path: &PathBuf) -> Result<Vec<Requirement>> {
    let file = File::open(requirements_path).unwrap();

    use std::io::{BufRead, BufReader};
    let reader = BufReader::new(file);

    let mut requirements = Vec::new();
    let mut errors = Vec::new();
    for (line_idx, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.

        use std::str::FromStr;
        match Requirement::from_str(line.as_str()) {
            Ok(requirement) => {
                requirements.push(requirement);
            }
            Err(err) => {
                errors.push((line_idx + 1, err));
            }
        };
    }

    if errors.len() > 0 {
        let mut lines = Vec::new();
        for (line_no, err) in errors {
            lines.push(format!("Line {}: {}", line_no, err));
        }
        bail!(
            "errors in parsing requirements file: \n{}",
            lines.join("\n")
        )
    }

    Ok(requirements)
}

// use super::config::OBLIGATED_PACKAGES;

fn get_obligated_requirements(installer: &Installer) -> Result<Vec<Requirement>> {
    let mut requirements = Vec::new();
    let mut errors = Vec::new();
    for (idx, requirement) in installer.obligated_requirements().iter().enumerate() {
        use std::str::FromStr;
        match Requirement::from_str(requirement) {
            Ok(requirement) => {
                requirements.push(requirement);
            }
            Err(err) => {
                errors.push((idx + 1, err));
            }
        };
    }

    if errors.len() > 0 {
        let mut lines = Vec::new();
        for (line_no, err) in errors {
            lines.push(format!("Line {}: {}", line_no, err));
        }
        bail!(
            "errors in parsing requirements file: \n{}",
            lines.join("\n")
        )
    }

    Ok(requirements)
}
