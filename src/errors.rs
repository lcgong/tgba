use thiserror::Error;

#[derive(Debug)]
pub enum DownloadingErrorKind {
    ServerError,
    NotFound,
    Timeout,
    Other,
}


#[derive(Error, Debug)]
pub struct DownloadingError {
    kind: DownloadingErrorKind,
    message: String,
}

impl DownloadingError {
    pub fn not_found(message: String) -> Self {
        DownloadingError {
            kind: DownloadingErrorKind::NotFound,
            message,
        }
    }

    pub fn timeout_error(message: String) -> Self {
        DownloadingError {
            kind: DownloadingErrorKind::Timeout,
            message,
        }
    }

    pub fn server_error(message: String) -> Self {
        DownloadingError {
            kind: DownloadingErrorKind::ServerError,
            message,
        }
    }

    pub fn error(message: String) -> Self {
        DownloadingError {
            kind: DownloadingErrorKind::Other,
            message,
        }
    }
}

impl std::fmt::Display for DownloadingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let kind_name = match self.kind {
            DownloadingErrorKind::ServerError => "网络服务错误",
            DownloadingErrorKind::NotFound => "无此资源",
            DownloadingErrorKind::Timeout => "网络超时",
            DownloadingErrorKind::Other => {
                return write!(f, "{}", self.message);
            }
        };

        write!(f, "{kind_name}：{}", self.message)
    }
}
