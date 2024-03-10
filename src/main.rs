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
use clap;

fn init_log(prog: &str) -> Result<()> {
    use log::LevelFilter;
    use log4rs::append::console::ConsoleAppender;
    use log4rs::append::file::FileAppender;
    use log4rs::config::{Appender, Config, Root};
    use log4rs::encode::pattern::PatternEncoder;

    let mut log_path: std::path::PathBuf = std::env::current_exe()?;
    log_path.pop();
    log_path.push(format!("{}.安装日志[可删除].txt", prog));

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
    log::info!("log initialized");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // let _result = std::panic::catch_unwind(|| {
    //     std::panic::set_hook(Box::new(|panic_info| {
    //         use std::io::Write;

    //         let mut file = std::fs::File::create("tgba-installer.程序意外退出.log")
    //             .expect("Failed to create panic.log");
    //         writeln!(file, "Panic: {}", panic_info).expect("Failed to write to panic log");
    //     }));
    // });

    let prog = std::env::args().nth(0).unwrap();
    let prog = std::path::Path::new(&prog)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap();

    init_log(prog)?;

    let args = clap::Command::new("tgba-installer")
        .arg(
            clap::Arg::new("py38")
                .long("3.8")
                .action(clap::ArgAction::SetTrue)
                .help("python 3.8"),
        )
        .get_matches();
    let flag_legacy_py38 = args.get_flag("py38");

    log::info!("start creating app");
    let mut app = if flag_legacy_py38 {
        myapp::MyApp::new(Some("3.8".to_string()))
    } else {
        myapp::MyApp::new(None)
    };

    app.run();

    Ok(())
}


