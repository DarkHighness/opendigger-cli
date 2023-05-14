#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error(transparent)]
    RequestError(#[from] reqwest::Error),
    #[error("Data not found: {0}")]
    DataNotFound(String),
    #[error("Request response error, status code: {0}")]
    BadRequestResponse(http::StatusCode),
}

impl ApiError {
    pub fn is_data_not_found(&self) -> bool {
        matches!(self, Self::DataNotFound(_))
    }
}
