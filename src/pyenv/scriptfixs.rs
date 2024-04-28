use super::Installer;
use anyhow::{bail, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use std::fs::File;

pub fn fix_patches(installer: &Installer) -> Result<()> {
    fix_win_activate_scripts(installer)?;

    fix_matplotlibrc(installer)?;

    disable_labtensions(installer)?;

    disable_lsp_diagnostics(installer)?;

    fix_launcher_logo_svg(installer)?;

    Ok(())
}

pub fn clean_cached_dir(installer: &Installer) -> Result<()> {

    let cached_dir = installer.cached_packages_dir.clone();

    if cached_dir.exists() {
        std::fs::remove_dir_all(cached_dir)?;
    }

    Ok(())
}

static SCRIPT_PROPMT_REGEX1: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"(set PROMPT=).*(%PROMPT%)"#).unwrap());
static SCRIPT_PROPMT_REGEX2: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"(set VIRTUAL_ENV_PROMPT=).*"#).unwrap());

/// 将虚拟环境提示符改为(TGBA)
fn fix_win_activate_scripts(installer: &Installer) -> Result<()> {
    static PROMPT: &'static str = "TGBA ";

    let mut script_path = installer.venv_dir.clone();
    script_path.extend(["Scripts", "activate.bat"]);

    let file = File::open(&script_path).unwrap();

    use std::io::{BufRead, BufReader};
    let reader = BufReader::new(file);

    let mut lines: Vec<String> = Vec::new();
    for line in reader.lines() {
        let Ok(mut line) = line else {
            bail!("从文件读取文本行错误")
        };

        if let Some(caps) = SCRIPT_PROPMT_REGEX1.captures(&line) {
            line = format!("{}{}{} ", &caps[1], PROMPT, &caps[2]);
            println!("{}", line);
        } else if let Some(caps) = SCRIPT_PROPMT_REGEX2.captures(&line) {
            line = format!("{}{}", &caps[1], PROMPT);
            println!("{}", line);
        };

        lines.push(line);
    }

    use std::io::Write;
    let mut file = File::create(&script_path)?;
    for line in lines {
        if let Err(err) = writeln!(file, "{}", line) {
            bail!("写入文件{}错误: {}", script_path.display(), err);
        }
    }

    Ok(())
}

static SANS_FONTS: [&str; 8] = [
    "Noto Sans CJK SC",
    "Microsoft YaHei",
    "SimHei",
    "DejaVu Sans",
    "Lucida Sans Unicode",
    "Arial",
    "Helvetica",
    "sans-serif",
];

static FONTFAMILY_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"#?(font\.family:.*)"#).unwrap() //
});

static SANSFAMILY_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"#?(font\.sans-serif:).*"#).unwrap());

fn fix_matplotlibrc(installer: &Installer) -> Result<()> {
    let mut rcfile_path = installer.venv_dir.clone();
    rcfile_path.extend([
        "Lib",
        "site-packages",
        "matplotlib",
        "mpl-data",
        "matplotlibrc",
    ]);

    let rcfile = File::open(&rcfile_path).unwrap();
    use std::io::{BufRead, BufReader};

    let mut lines: Vec<String> = Vec::new();
    for line in BufReader::new(rcfile).lines() {
        let Ok(mut line) = line else {
            bail!("err: read a line")
        };

        if let Some(caps) = FONTFAMILY_REGEX.captures(&line) {
            line = format!("{}", &caps[1]);
            println!("{}", line);
        } else if let Some(caps) = SANSFAMILY_REGEX.captures(&line) {
            line = format!("{} {}", &caps[1], SANS_FONTS.join(", "));
            println!("{}", line);
        }

        lines.push(line);
    }

    use std::io::Write;
    let mut file = File::create(&rcfile_path)?;
    for line in lines {
        if let Err(err) = writeln!(file, "{}", line) {
            bail!("写入文件{}错误: {}", rcfile_path.display(), err);
        }
    }

    Ok(())
}

fn disable_labtensions(installer: &Installer) -> Result<()> {
    let mut labconfig_path = installer.venv_dir.clone();
    labconfig_path.extend(["etc", "jupyter", "labconfig"]);
    if let Err(err) = std::fs::create_dir_all(&labconfig_path) {
        bail!(
            "创建jupyterlab配置目录{}失败: {}",
            labconfig_path.display(),
            err
        )
    }
    labconfig_path.push("page_config.json");

    let labconfig = r#"
{
    "disabledExtensions": {
        "@jupyterlab/cell-toolbar-extension": true,
        "@jupyterlab/debugger-extension": true
    }
}    
"#;

    use std::io::Write;
    let mut file = File::create(&labconfig_path)?;
    if let Err(err) = writeln!(file, "{}", labconfig) {
        bail!("写入文件{}错误: {}", labconfig_path.display(), err);
    }

    Ok(())
}

fn disable_lsp_diagnostics(installer: &Installer) -> Result<()> {
    // https://jupyterlab.readthedocs.io/en/stable/user/directories.html#overrides-json

    let mut settings_path = installer.venv_dir.clone();
    settings_path.extend(["share", "jupyter", "lab", "settings"]);
    if let Err(err) = std::fs::create_dir_all(&settings_path) {
        bail!(
            "创建jupyterlab配置目录{}失败: {}",
            settings_path.display(),
            err
        )
    }
    settings_path.push("overrides.json");

    let config = r#"
    {
        "@jupyter-lsp/jupyterlab-lsp:diagnostics": {
            "defaultSeverity": "Error",
            "disable": true
        }
    } 
"#;

    use std::io::Write;
    let mut file = File::create(&settings_path)?;
    if let Err(err) = writeln!(file, "{}", config) {
        bail!("写入文件{}错误: {}", settings_path.display(), err);
    }

    Ok(())
}

fn fix_launcher_logo_svg(installer: &Installer) -> Result<()> {
    let mut logo_svg_file = installer.venv_dir.clone();
    logo_svg_file.extend(["share", "jupyter", "kernels", "python3", "logo-svg.svg"]);

    if logo_svg_file.exists() {
        std::fs::remove_file(logo_svg_file)?;
        log::info!("删除logo-svg.svg，解决launcher无法正常显示Bug");
    }

    Ok(())
}
