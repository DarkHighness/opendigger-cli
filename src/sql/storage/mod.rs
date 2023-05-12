use std::{collections::HashMap, fmt::Debug};

use super::{table::StorageTable, TableType};

mod alter_table;
mod index;
mod store;
mod transaction;

#[derive(Debug)]
pub struct Storage {
    strategy: Box<dyn crate::sql::StorageStrategy>,
    tables: std::sync::Mutex<HashMap<TableType, StorageTable>>,
}

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error(transparent)]
    ApiError(#[from] crate::api::ApiError),
    #[error(transparent)]
    StrategyError(#[from] anyhow::Error),
    #[error("Type is not supported: {0}{1}")]
    UnsupportedTableType(String, crate::api::Type),
}

impl Storage {
    pub async fn build_from_strategy(
        strategy: Box<dyn crate::sql::StorageStrategy>,
    ) -> Result<Self, StorageError> {
        let storage = Self {
            strategy,
            tables: Default::default(),
        };

        Ok(storage)
    }

    pub async fn fetch_table(
        &self,
        table_type: TableType,
    ) -> Result<Option<StorageTable>, StorageError> {
        let guard = self.tables.lock().unwrap();

        if guard.contains_key(&table_type) {
            return Ok(guard.get(&table_type).cloned());
        }

        drop(guard);

        let table = self
            .strategy
            .fetch_table(table_type)
            .await
            .map_err(|err| StorageError::StrategyError(err))?;

        let mut guard = self.tables.lock().unwrap();

        guard.insert(table_type, table);

        Ok(guard.get(&table_type).cloned())
    }
}
