// #![windows_subsystem = "windows"]
// 在debug模式下终端显示print，发行版不显示终端窗口
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod dialog;
pub mod errors;
pub mod myapp;
pub mod pyenv;
pub mod resources;
pub mod status;
pub mod steps;
pub mod style;
pub mod utils;

use anyhow::Result;

fn init_log() -> Result<()> {
    use log::LevelFilter;
    use log4rs::append::console::ConsoleAppender;
    use log4rs::append::file::FileAppender;
    use log4rs::config::{Appender, Config, Root};
    use log4rs::encode::pattern::PatternEncoder;

    let mut log_path: std::path::PathBuf = std::env::current_exe()?;
    log_path.pop();
    log_path.push("TGBA安装日志.log.txt");

    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} {l} {t} - {m}{n}")))
        .build(log_path)?;

    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} {m}{n}")))
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(
            Root::builder()
                .appender("stdout")
                .appender("logfile")
                .build(LevelFilter::Info),
        )?;

    log4rs::init_config(config)?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    init_log()?;

    let mut app = myapp::MyApp::new();
    app.run();

    Ok(())
}

// pub async fn cmd_main() -> Result<()> {
//     use pyenv::Installer;
//     let target_dir = std::env::current_dir()?;

//     use pyenv::{create_winlnk, fix_patches};
//     use pyenv::{ensure_python_venv, install_requirements};

//     let mut installer = Installer::new(target_dir)?;

//     ensure_python_venv(&mut installer).await?;
//     install_requirements(&installer).await?;
//     create_winlnk(&installer, &installer.target_dir())?;
//     fix_patches(&installer)?;

//     Ok(())
// }
