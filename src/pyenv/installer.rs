use anyhow::{bail, Error, Result};
use reqwest;
use std::collections::HashMap;
use std::path::PathBuf;

use super::config::{get_cpytion_candidates, get_pip_user_agent};
use super::utils::parse_version;

pub struct Installer {
    target_dir: PathBuf,
    tgba_dir: PathBuf,

    pub(crate) python_version: String,
    pub(crate) python_version_full: String,
    pub(crate) cached_packages_dir: PathBuf,
    pub(crate) pydist_dir: PathBuf,
    pub(crate) pydist_source: (&'static str, &'static str),
    pub(crate) venv_dir: PathBuf,
    pub(crate) venv_python_path: PathBuf,
    pub(crate) client: reqwest::Client,
    pub platform_tag: Option<String>,
    pub support_tags_map: HashMap<String, u32>,
}

impl Installer {
    pub fn new(target_dir: PathBuf) -> Result<Self, Error> {
        let tgba_dir = target_dir.join(".tgba");

        let (py_ver, dist_url, dist_digest) = get_cpytion_candidates()?;
        let python_version = parse_version(py_ver)?;

        let nums = &python_version.release;
        if nums.len() < 3 {
            bail!("Python版本号不全: major.minor.micro")
        }

        let python_version = format!("{}.{}", &nums[0], &nums[1]);
        let python_version_full = format!("{}.{}.{}", &nums[0], &nums[1], &nums[2]);

        let py_dist_dir = tgba_dir.join(format!("cpython-{}", python_version_full));
        let py_venv_dir = tgba_dir.join(format!("venv"));
        let cached_packages_dir = tgba_dir.join("cached_packages");

        let mut venv_python_path = py_venv_dir.clone();
        venv_python_path.push("Scripts");
        venv_python_path.push("python.exe");

        let client = reqwest::Client::builder()
            .user_agent(get_pip_user_agent())
            .build()?;

        Ok(Installer {
            target_dir,
            python_version,
            python_version_full,
            tgba_dir,
            pydist_dir: py_dist_dir,
            pydist_source: (dist_url, dist_digest),
            venv_python_path,
            venv_dir: py_venv_dir,
            cached_packages_dir,
            client,
            platform_tag: None,
            support_tags_map: HashMap::new(),
        })
    }

    pub fn target_dir(&self) -> &PathBuf {
        &self.target_dir
    }

    pub fn tgba_dir(&self) -> &PathBuf {
        &self.tgba_dir
    }

    pub fn log(&self, msg: &str) {
        println!("{}", msg);
    }
}

pub async fn main() -> Result<()> {
    let target_dir = std::env::current_dir()?;

    let mut installer = Installer::new(target_dir)?;

    use super::venv::ensure_python_venv;
    ensure_python_venv(&mut installer).await?;

    use super::requirements::install_requirements;
    install_requirements(&installer).await?;

    use super::winlnk::create_winlnk;
    create_winlnk(&installer, &installer.target_dir.clone())?;

    Ok(())
}
