// #![windows_subsystem = "windows"]
// 在debug模式下终端显示print，发行版不显示终端窗口
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// mod heading;
// mod step;
// mod style;
// mod winapp;
pub mod pypip;

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


fn get_pip_user_agent() -> String {
    // pip/23.2.1 {"ci":null,"cpu":"AMD64","implementation":{"name":"CPython","version":"3.11.4"},"installer":{"name":"pip","version":"23.2.1"},"openssl_version":"OpenSSL 1.1.1u  30 May 2023","python":"3.11.4","rustc_version":"1.72.1","setuptools_version":"65.5.0","system":{"name":"Windows","release":"10"}}
    format!("pip/{}", PIP_VERSION)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = reqwest::Client::builder()
        .user_agent(get_pip_user_agent())
        .build()?;

    

    // download(&client).await?;

    
    use pypip::project::get_project_index;
    get_project_index(&client).await?;

    Ok(())
}
