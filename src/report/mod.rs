mod repo;

pub use repo::RepoOverview;
use std::io;

#[derive(Debug, thiserror::Error)]
pub enum ReportError {
    #[error(transparent)]
    ApiError(#[from] crate::api::ApiError),
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}
