// #![windows_subsystem = "windows"]
// 在debug模式下终端显示print，发行版不显示终端窗口
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// mod heading;
// mod step;
// mod style;
// mod winapp;
pub mod pypip;
pub mod pyenv;

// use winapp::main_app;

// fn main() {

//     pypip::pyutils::main();

// }
use reqwest;
use reqwest::header::{HeaderMap, HeaderValue};
use std::fs::File;
use std::io::{self, Write};
// use indicatif::{ProgressBar, ProgressStyle};

const PIP_VERSION: &'static str = "23.2.1";

use anyhow::{anyhow, Error, Ok};

async fn download(client: &reqwest::Client) -> Result<(), Error> {
    // let url = "https://pypi.tuna.tsinghua.edu.cn/packages/3b/43/2368d8ffee6e33f282f548d42fa222bd385cc9f66545b260e7d08e90046b/jupyterlab-4.0.6-py3-none-any.whl#sha256=7d9dacad1e3f30fe4d6d4efc97fda25fbb5012012b8f27cc03a2283abcdee708";
    let url = "https://mirrors.bfsu.edu.cn/pypi/web/packages/3b/43/2368d8ffee6e33f282f548d42fa222bd385cc9f66545b260e7d08e90046b/jupyterlab-4.0.6-py3-none-any.whl#sha256=7d9dacad1e3f30fe4d6d4efc97fda25fbb5012012b8f27cc03a2283abcdee708";

    let mut resp = client.get(url).send().await?;

    let total_size = resp
        .content_length()
        .ok_or(anyhow!("Failed to get content length from '{}'", &url))?;

    // let mut dest = File::create("downloaded_file.ext")?;
    let mut downloaded = 0;
    //
    // println!("{}", response.text().await?);

    while let Some(chunk) = resp.chunk().await? {
        // dest.write_all(&chunk)?;
        downloaded += chunk.len() as u64;
        println!("downloaded: {} / {}", downloaded, total_size);
        // progress_bar.set_position(downloaded);
        break;
    }

    println!("done!");
    Ok(())
}




#[tokio::main]
async fn main() -> Result<(), Error> {
    // use pyenv::installer::get_pip_user_agent;
    
    // let client = reqwest::Client::builder()
    //     .user_agent(get_pip_user_agent())
    //     .build()?;
    // use pyenv::index::get_project_index;
    // get_project_index(&client).await?;

    pyenv::installer::main().await?;

    // use url::Url; // 2.1.0

// fn main() {
    // let u = Url::parse("http://my.com/dir1/dir2/simple").unwrap();
    // let u2 = u.join("abc/").unwrap();
    // println!("{}", u2);
    // assert_eq!("/dir1/dir2/", u2.path());
// }

    // download(&client).await?;

    
    // use pyenv::index::get_project_index;
    // get_project_index(&client).await?;

    // println!("{}", "a__b_c".replace("_", "-"));

    Ok(())
}
