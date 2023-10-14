use std::path::PathBuf;

use anyhow::{bail, Result};
use mslnk::ShellLink;

use super::installer::Installer;

static ICON_FILE_NAME: &str = "tgba-jupyterlab-48x48.ico";

pub fn create_winlnk(installer: &Installer, work_dir: &PathBuf) -> Result<()> {
    copy_jupyterlab_icon(installer)?;

    create_shell_lnk(installer, work_dir)?;
    create_jupyterlab_lnk(installer, work_dir)?;

    Ok(())
}

fn create_shell_lnk(installer: &Installer, work_dir: &PathBuf) -> Result<()> {
    let mut lnk = match ShellLink::new(r"C:\Windows\System32\cmd.exe") {
        Ok(lnk) => lnk,
        Err(err) => bail!("创建: {}", err),
    };

    let mut activate_script_path = installer.venv_dir.clone();
    activate_script_path.push("Scripts");
    activate_script_path.push("activate.bat");

    lnk.set_name(Some("TGBA Shell".to_string()));
    lnk.set_arguments(Some(format!(
        r#"/K "{}"#,
        activate_script_path.to_string_lossy()
    )));

    lnk.set_icon_location(Some(
        installer
            .tgba_dir()
            .join(ICON_FILE_NAME)
            .to_string_lossy()
            .to_string(),
    ));
    lnk.set_working_dir(Some(work_dir.to_string_lossy().to_string()));

    let Err(err) = lnk.create_lnk(work_dir.join("TGBAShell.lnk")) else {
        return Ok(());
    };

    bail!("创建快捷错误: {}", err);
}

fn create_jupyterlab_lnk(installer: &Installer, work_dir: &PathBuf) -> Result<()> {
    let mut lnk = match ShellLink::new(installer.venv_python_path.to_string_lossy().as_ref()) {
        Ok(lnk) => lnk,
        Err(err) => bail!("创建: {}", err),
    };

    let mut activate_script_path = installer.venv_dir.clone();
    activate_script_path.push("Scripts");
    activate_script_path.push("activate.bat");

    lnk.set_name(Some("TGBA JupyterLab".to_string()));
    lnk.set_arguments(Some("-m jupyterlab".to_string()));

    lnk.set_icon_location(Some(
        installer
            .tgba_dir()
            .join(ICON_FILE_NAME)
            .to_string_lossy()
            .to_string(),
    ));
    lnk.set_working_dir(Some(work_dir.to_string_lossy().to_string()));

    let Err(err) = lnk.create_lnk(work_dir.join("TGBAJupyterLab.lnk")) else {
        return Ok(());
    };

    bail!("创建快捷错误: {}", err);
}

use super::super::resources::RESOURCES;

fn copy_jupyterlab_icon(installer: &Installer) -> Result<()> {
    use std::fs::File;

    let mut icon_file = File::create(installer.tgba_dir().join(ICON_FILE_NAME))?;

    use std::io::Write;
    icon_file.write_all(RESOURCES.get_app_icon())?;

    Ok(())
}
