use anyhow::{anyhow, Result};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pip_version: String,
    pypi: Vec<PyPIMirror>,
    cpython: Vec<CPythonDistSource>,
    obligated_requirements: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PyPIMirror {
    name: String,
    url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CPythonDistSource {
    python_version: String,
    version: String,
    url: String,
    checksum: String,
}

impl Config {
    pub fn load() -> Result<Config> {
        use super::super::resources::RESOURCES;
        let config: Config = toml::from_str(RESOURCES.get_config_toml())?;

        Ok(config)
    }

    pub fn pip_version(&self) -> &str {
        &self.pip_version
    }

    pub fn get_cpytion_source(
        &self,
        mut python_version: Option<String>,
    ) -> Result<&CPythonDistSource> {
        if python_version.is_none() {
            use super::utils::get_windows_major_versoin;
            let win_major = get_windows_major_versoin()?;
            if win_major == 7 {
                python_version = Some("3.8".to_string());
            }
        }

        if self.cpython.is_empty() {
            return Err(anyhow!("在配置文件无[[cpython]]配置信息"));
        }

        match python_version {
            Some(python_version) => {
                for dist in &self.cpython {
                    if dist.python_version == python_version {
                        return Ok(dist);
                    }
                }

                Err(anyhow!("在安装配置文件没找到{}下载信息", python_version))
            }
            None => Ok(self.cpython.first().unwrap()),
        }
    }

    pub fn get_pypi_mirrors(&self) -> &[PyPIMirror] {
        &self.pypi
    }

    pub fn obligated_requirements(&self) -> &[String] {
        &self.obligated_requirements
    }
}

impl PyPIMirror {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn package_url(&self, canonical_name: &str) -> String {
        if self.url.ends_with('/') {
            format!("{}{}/", self.url, canonical_name)
        } else {
            format!("{}/{}/", self.url, canonical_name)
        }
    }
}

impl CPythonDistSource {
    pub fn cpython_version(&self) -> &str {
        &self.version
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn checksum(&self) -> &str {
        &self.checksum
    }
}
