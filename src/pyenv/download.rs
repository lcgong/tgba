use anyhow::{bail, Result};
use std::time::{Duration, Instant};

use super::installer::Installer;

impl Installer {
    pub fn new_downloading_job(&self) -> DownloadingStatus {
        DownloadingStatus::new()
    }
}

pub async fn download(installer: &Installer, url: &str) -> Result<Vec<u8>> {
    let mut resp = installer.client.get(url).send().await?;

    let http_status = resp.status();
    if !http_status.is_success() {
        bail!("Failed [{}] to download: {}", http_status.as_u16(), url)
    }

    let Some(total_size) = resp.content_length() else {
        bail!("Failed to get content length from '{}'", &url)
    };

    let mut buffer: Vec<u8> = Vec::new();

    let mut status = installer.new_downloading_job();

    // let mut dest = File::create("downloaded_file.ext")?;
    // dest.write_all(&chunk)?;

    status.start_downloading(total_size);

    while let Some(chunk) = resp.chunk().await? {
        use std::io::Write;
        buffer.write_all(&chunk).unwrap();

        status.download(chunk.len() as u64);
    }

    status.finish();

    Ok(buffer)
}


pub struct DownloadingStatus {
    start_time: Instant,
    elasped: Option<Duration>,
    total_size: Option<u64>,
    downloaded: Option<u64>,
}

impl DownloadingStatus {
    pub fn new() -> Self {
        DownloadingStatus {
            start_time: Instant::now(),
            elasped: None,
            total_size: None,
            downloaded: None,
        }
    }

    pub fn start_downloading(&mut self, total_size: u64) {
        self.start_time = Instant::now();
        self.total_size = Some(total_size);
        self.downloaded = Some(0);
    }

    pub fn download(&mut self, size: u64) {
        if let Some(downloaded) = self.downloaded {
            self.downloaded = Some(downloaded + size);
        }
    }

    pub fn finish(&mut self) {
        self.elasped = Some(self.start_time.elapsed());
    }
}
