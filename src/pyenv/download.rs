use super::super::status::{DownloadingStats, StatusUpdate};
use super::installer::Installer;
use super::super::errors::DownloadingError;

pub async fn download(
    installer: &Installer,
    status_updater: &impl StatusUpdate,
    url: &str,
    title: &str,
) -> Result<Vec<u8>, DownloadingError> {
    let mut resp = match installer.client.get(url).send().await {
        Ok(resp) => resp,
        Err(err) => {
            return Err(if err.is_timeout() {
                DownloadingError::timeout_error(format!("下载{}", url))
            } else {
                DownloadingError::server_error(format!("{}", err))
            });
        }
    };

    let http_status = resp.status();
    if !http_status.is_success() {
        let status_code = http_status.as_u16();
        if status_code == 404 {
            return Err(DownloadingError::not_found(format!("{}", url)));
        } else {
            return Err(DownloadingError::server_error(format!(
                "HTTP状态码[{}]",
                status_code
            )));
        }
    }

    let Some(total_size) = resp.content_length() else {
        return Err(DownloadingError::error(format!(
            "HTTP响应异常，无内容长度信息：{url}",
        )));
    };

    let mut buffer: Vec<u8> = Vec::new();

    let mut stats = DownloadingStats::new(title, total_size);
    status_updater.update_downloading(&stats);
    loop {
        match resp.chunk().await {
            Ok(Some(chunk)) => {
                use std::io::Write;
                if let Err(err) = buffer.write_all(&chunk) {
                    return Err(DownloadingError::server_error(format!(
                        "下载写入缓存错误：{}",
                        err
                    )));
                };

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
                return Err(if err.is_timeout() {
                    DownloadingError::timeout_error(format!("下载{}", url))
                } else {
                    DownloadingError::server_error(format!("{}", err))
                });
            }
        };
    }

    stats.finish();
    status_updater.update_downloading(&stats);

    Ok(buffer)
}

// pub async fn fetch_text(installer: &Installer, url: &str, _title: &str) -> Result<String> {
//     let resp = match installer.client.get(url).send().await {
//         Ok(resp) => resp,
//         Err(err) => bail!("{}", err),
//     };

//     let text = match resp.text().await {
//         Ok(text) => text,
//         Err(err) => bail!("{}", err),
//     };

//     Ok(text)
// }
