// #![windows_subsystem = "windows"]
// 在debug模式下终端显示print，发行版不显示终端窗口
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// mod heading;
pub mod resources;
pub mod step;
pub mod style;
// mod winapp;
pub mod pyenv;

// pub mod pypip;

// use winapp::main_app;

// fn main() {

//     pypip::pyutils::main();

// }
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // use crate::pyenv::config::Config;
    // Config::load()?;

    // pyenv::installer::main().await?;

    // download(&client).await?;

    // use pyenv::index::get_project_index;
    // get_project_index(&client).await?;

    // println!("{}", "a__b_c".replace("_", "-"));

    let target_dir = std::env::current_dir()?;

    use pyenv::{create_winlnk, ensure_python_venv, install_requirements, Installer};

    let mut installer = Installer::new(target_dir)?;

    ensure_python_venv(&mut installer).await?;
    install_requirements(&installer).await?;
    create_winlnk(&installer, &installer.target_dir())?;

    Ok(())
}
