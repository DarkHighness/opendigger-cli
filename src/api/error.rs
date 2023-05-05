#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Data not found: {0}")]
    DataNotFound(String),
    #[error("Reqwest response error, status code: {0}")]
    BadReqwestResponse(http::StatusCode),
}
