use anyhow::{bail, Result};
use pep508_rs::Requirement;
use std::fs::File;

use super::installer::Installer;

pub async fn install_requirements(installer: &Installer) -> Result<()> {
    use super::index::project::PyPi;
    let pypi = PyPi::new("清华源", "https://pypi.tuna.tsinghua.edu.cn/simple");

    let cached_packages_dir = &installer.cached_packages_dir;
    if let Err(_err) = std::fs::create_dir_all(cached_packages_dir) {
        bail!("创建目录{}失败: {}", cached_packages_dir.display(), _err)
    }

    use super::index::project::download_requirement;
    for requirement in &get_requirements(&installer).await? {
        // if !requirement.name.starts_with("astroid") {
        //     continue;
        // }
        
        download_requirement(installer, &pypi, requirement).await?;
        // break;
    }

    Ok(())
}

async fn get_requirements(installer: &Installer) -> Result<Vec<Requirement>> {
    let requirements_filename = format!(
        "requirements-{}-{}.txt",
        installer.python_version,
        installer.platform_tag.as_ref().unwrap()
    );

    let requirements_file = installer.venv_dir.join(requirements_filename);
    if !requirements_file.is_file() {
        bail!("unimplemented: {}", requirements_file.display())
    }

    let file = File::open(requirements_file).unwrap();

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
