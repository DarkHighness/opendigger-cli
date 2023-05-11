use crate::sql::AggregateTableEntry;

use futures::future;

use std::fmt::Debug;

use self::tables::StorageTable;

mod alter_table;
mod index;
mod store;
pub mod tables;
mod transaction;

#[derive(Debug)]
pub struct Storage {
    pub tables: Vec<Box<dyn StorageTable>>,
}

#[derive(Debug, thiserror::Error)]
pub enum StorageBuildError {
    #[error(transparent)]
    ApiError(#[from] crate::api::ApiError),
    #[error("Errors: {0:?}")]
    MultiError(Vec<StorageBuildError>),
    #[error("Type is not supported: {0}{1}")]
    UnsupoortedType(String, crate::api::Type),
}

impl Storage {
    pub async fn build_from_entries(
        entries: &[AggregateTableEntry],
    ) -> Result<Self, StorageBuildError> {
        let tables = future::join_all(
            entries
                .iter()
                .map(async move |entry| entry.to_storage_table().await),
        )
        .await;

        let (tables, errors): (Vec<_>, Vec<_>) = tables.into_iter().partition(Result::is_ok);

        let (tables, errors): (Vec<_>, Vec<_>) = (
            tables.into_iter().map(Result::unwrap).collect(),
            errors.into_iter().map(Result::unwrap_err).collect(),
        );

        if errors.is_empty() {
            Ok(Self { tables })
        } else if errors.len() == 1 {
            Err(errors.into_iter().next().unwrap())
        } else {
            Err(StorageBuildError::MultiError(errors))
        }
    }
}
