
use once_cell::sync::OnceCell;

pub struct Resources {
    app_icon: OnceCell<&'static [u8]>,
    config_toml: OnceCell<String>,
    requirements: OnceCell<&'static [u8]>,
    requirements_legacy: OnceCell<&'static [u8]>,
}

impl Resources {
    pub const fn new() -> Resources {
        Resources {
            app_icon: OnceCell::new(),
            config_toml: OnceCell::new(),
            requirements: OnceCell::new(),
            requirements_legacy: OnceCell::new(),
        }
    }

    pub fn get_requirements_file(&self, python_version: &str) -> &[u8] {
        if python_version != "3.8" {
            self.requirements
                .get_or_init(|| include_bytes!("../requirements/requirements-win.txt"))
        } else {
            self.requirements_legacy
                .get_or_init(|| include_bytes!("../requirements/requirements-win-py38.txt"))
        }
    }

    pub fn get_app_icon(&self) -> &[u8] {
        self.app_icon.get_or_init(|| {
            let data = include_bytes!("../resources/tgba-jupyterlab-48x48.ico");
            data
        })
    }

    pub fn get_config_toml(&self) -> &str {
        self.config_toml.get_or_init(|| {
            let data = include_bytes!("../requirements/config.toml");
            String::from_utf8_lossy(data).to_string()
        })
    }
}

pub static RESOURCES: Resources = Resources::new();
