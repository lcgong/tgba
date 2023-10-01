// #![windows_subsystem = "windows"]
// 在debug模式下终端显示print，发行版不显示终端窗口
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// mod heading;
// mod step;
// mod style;
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

    pyenv::installer::main().await?;


    // download(&client).await?;

    // use pyenv::index::get_project_index;
    // get_project_index(&client).await?;

    // println!("{}", "a__b_c".replace("_", "-"));

    Ok(())
}
