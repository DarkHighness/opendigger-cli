mod repo;

pub use repo::ReporOverview;

#[derive(Debug, thiserror::Error)]
pub enum ReportError {
    #[error(transparent)]
    ApiError(#[from] crate::api::ApiError),
}
