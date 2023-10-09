use anyhow::{bail, Result};

use super::super::status::{DownloadingStats, StatusUpdate};
use super::installer::Installer;

pub async fn download(
    installer: &Installer,
    status_updater: &impl StatusUpdate,
    url: &str,
    title: &str,
) -> Result<Vec<u8>> {
    let mut resp = installer.client.get(url).send().await?;

    let http_status = resp.status();
    if !http_status.is_success() {
        bail!("Failed [{}] to download: {}", http_status.as_u16(), url)
    }

    let Some(total_size) = resp.content_length() else {
        bail!("Failed to get content length from '{}'", &url)
    };

    let mut buffer: Vec<u8> = Vec::new();

    let mut stats = DownloadingStats::new(title, total_size);
    status_updater.update_downloading(&stats);
    loop {
        match resp.chunk().await {
            Ok(Some(chunk)) => {
                use std::io::Write;
                buffer.write_all(&chunk).unwrap();

                stats.update(chunk.len() as u64);
                if stats.out_of_tick() {
                    status_updater.update_downloading(&stats);
                    stats.next_tick();
                }
            }
            Ok(None) => {
                break;
            }
            Err(err) => {
                if err.is_timeout() {}
                return Err(err.into());
            }
        };
    }

    stats.finish();
    status_updater.update_downloading(&stats);

    Ok(buffer)
}

pub async fn fetch_text(installer: &Installer, url: &str, title: &str) -> Result<String> {
    installer.log(title);

    let resp = match installer.client.get(url).send().await {
        Ok(resp) => resp,
        Err(err) => bail!("{}", err),
    };

    let text = match resp.text().await {
        Ok(text) => text,
        Err(err) => bail!("{}", err),
    };

    Ok(text)
}
