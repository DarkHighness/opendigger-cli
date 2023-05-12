use async_trait::async_trait;
use gluesql::core::ast;

use crate::sql::TableType;

use super::{StorageStrategy, StorageStrategyError};

#[derive(Debug)]
pub struct BruteForceStoragePolicy {
    owners: Vec<String>,
}

#[async_trait(? Send)]
impl StorageStrategy for BruteForceStoragePolicy {
    async fn analyze_query(
        &mut self,
        _query: &str,
        _statements: &[ast::Statement],
    ) -> Result<(), StorageStrategyError> {
        Ok(())
    }

    async fn fetch_table(
        &self,
        _table_type: TableType,
    ) -> Result<crate::sql::table::StorageTable, StorageStrategyError> {
        unimplemented!()
    }
}

impl BruteForceStoragePolicy {
    pub fn new() -> Self {
        Self {
            owners: Default::default(),
        }
    }
}
