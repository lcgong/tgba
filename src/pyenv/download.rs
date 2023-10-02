use anyhow::{bail, Result};
use std::time::{Duration, Instant};

use super::installer::Installer;

impl Installer {
    pub fn start_downloading(&self, title: &str, total_size: u64) -> DownloadingStats {
        DownloadingStats::new(self, title, total_size)
    }
}

pub async fn download(installer: &Installer, url: &str, title: &str) -> Result<Vec<u8>> {
    let mut resp = installer.client.get(url).send().await?;

    let http_status = resp.status();
    if !http_status.is_success() {
        bail!("Failed [{}] to download: {}", http_status.as_u16(), url)
    }

    let Some(total_size) = resp.content_length() else {
        bail!("Failed to get content length from '{}'", &url)
    };

    let mut buffer: Vec<u8> = Vec::new();

    let mut stats = installer.start_downloading(title, total_size);

    loop {
        match resp.chunk().await {
            Ok(Some(chunk)) => {
                use std::io::Write;
                buffer.write_all(&chunk).unwrap();

                stats.update(chunk.len() as u64);
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

pub struct DownloadingStats<'a> {
    installer: &'a Installer,
    title: String,
    count: u64,
    start_time: Instant,
    total_size: u64,
    downloaded: u64,
    elasped: Option<Duration>,
    prev_start_time: Instant,
    prev_downloaded: u64,
    // queue: VecDeque<(Instant, u64)>,
}

impl<'a> DownloadingStats<'a> {
    pub fn new(installer: &'a Installer, title: &str, total_size: u64) -> Self {
        DownloadingStats {
            installer,
            title: title.to_string(),
            count: 0,
            start_time: Instant::now(),
            total_size,
            downloaded: 0,
            elasped: None,
            prev_start_time: Instant::now(),
            prev_downloaded: 0,
        }
    }

    pub fn update(&mut self, size: u64) {
        let now = Instant::now();

        if self.prev_start_time.elapsed().as_secs_f64() > 0.3 {
            self.prev_downloaded = self.downloaded;
            self.prev_start_time = self.start_time;

            self.count += 1;
            self.downloaded += size;
            self.start_time = now;

            self.log();
        } else {
            self.count += 1;
            self.downloaded += size;
            self.start_time = now;
        }
    }

    pub fn count(&self) -> u64 {
        self.count
    }

    pub fn downloaded(&self) -> u64 {
        self.downloaded
    }

    pub fn percentage(&self) -> f64 {
        if self.total_size != 0 {
            self.downloaded as f64 / (self.total_size as f64) * 100.0
        } else {
            0.0
        }
    }

    pub fn speed(&self) -> f64 {
        let downloaded = self.downloaded - self.prev_downloaded;
        let elapsed = self
            .start_time
            .duration_since(self.prev_start_time)
            .as_secs_f64();

        if elapsed.abs() < 1e-10 {
            0.0
        } else {
            downloaded as f64 / elapsed
        }
    }

    pub fn log(&self) {
        self.installer.log(
            format!(
                "{} size: {:5.2}% {}, speed: {:.2}",
                self.title,
                self.percentage(),
                self.downloaded(),
                self.speed()
            )
            .as_str(),
        );
    }

    pub fn finish(&mut self) {
        self.elasped = Some(self.start_time.elapsed());
        self.log();
    }
}
