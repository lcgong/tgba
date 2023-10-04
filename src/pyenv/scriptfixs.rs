use anyhow::{bail, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;

use super::Installer;

pub fn fix_patches(installer: &Installer) -> Result<()> {
    fix_win_activate_scripts(installer)?;
    fix_matplotlibrc(installer)?;

    Ok(())
}

/// 将虚拟环境提示符改为(TGBA)
pub fn fix_win_activate_scripts(installer: &Installer) -> Result<()> {
    lazy_static! {
        static ref PTN1: Regex = Regex::new(r#"(set PROMPT=).*(%PROMPT%)"#).unwrap();
        static ref PTN2: Regex = Regex::new(r#"(set VIRTUAL_ENV_PROMPT=).*"#).unwrap();
    }

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

        if let Some(caps) = PTN1.captures(&line) {
            line = format!("{}{}{} ", &caps[1], PROMPT, &caps[2]);
            println!("{}", line);
        } else if let Some(caps) = PTN2.captures(&line) {
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

pub fn fix_matplotlibrc(installer: &Installer) -> Result<()> {
    lazy_static! {
        static ref SANS_FONTS: Vec<&'static str> = vec![
            "Noto Sans CJK SC",
            "Microsoft YaHei",
            "SimHei",
            "DejaVu Sans",
            "Lucida Sans Unicode",
            "Arial",
            "Helvetica",
            "sans-serif",
        ];
        static ref FONTFAMILY_REGEX: Regex = Regex::new(r#"#?(font\.family:.*)"#).unwrap();
        static ref SANSFAMILY_REGEX: Regex = Regex::new(r#"#?(font\.sans-serif:).*"#).unwrap();
    }
    // let

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
