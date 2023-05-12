use async_trait::async_trait;
use gluesql::core::ast;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::sql::{
    table::{TableEntry, TableOwner},
    TableType,
};

use super::StorageStrategy;

pub static SINGLE_QUOTED_STRING_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"'([^']*)'"#).unwrap());

#[derive(Debug)]
pub struct BruteForceStoragePolicy {
    owners: Option<Vec<TableOwner>>,
}

#[async_trait(? Send)]
impl StorageStrategy for BruteForceStoragePolicy {
    async fn analyze_query(
        &mut self,
        query: &str,
        _statements: &[ast::Statement],
    ) -> anyhow::Result<()> {
        let owners: Vec<TableOwner> = SINGLE_QUOTED_STRING_REGEX
            .captures_iter(query)
            .filter_map(|capture| capture.get(1))
            .filter_map(|capture| TableOwner::new(capture.as_str()))
            .collect();

        if owners.is_empty() {
            anyhow::bail!("No owner found in query: {}", query);
        }

        tracing::debug!("Found owners: {:?}", owners);

        self.owners = Some(owners);

        Ok(())
    }

    async fn fetch_table(
        &self,
        table_type: TableType,
    ) -> anyhow::Result<crate::sql::table::StorageTable> {
        let owners = self.owners.as_ref().unwrap();
        let entries = owners
            .iter()
            .filter_map(|owner| TableEntry::new(table_type.clone(), owner.clone()))
            .collect::<Vec<_>>();

        let (tables, errors) =
            futures::future::join_all(entries.iter().map(|entry| entry.fetch_data()))
                .await
                .into_iter()
                .partition::<Vec<_>, _>(|result| result.is_ok());

        if !errors.is_empty() {
            anyhow::bail!("Failed to fetch data: {:?}", errors);
        }

        let tables = tables
            .into_iter()
            .map(|result| result.unwrap())
            .flatten()
            .collect();

        Ok(crate::sql::table::StorageTable::new(table_type, tables))
    }
}

impl BruteForceStoragePolicy {
    pub fn new() -> Self {
        Self {
            owners: Default::default(),
        }
    }
}
