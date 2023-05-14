use std::{cell::RefCell, collections::HashSet};

use async_trait::async_trait;
use gluesql::core::ast;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::sql::{
    table::{DataFetchError, TableEntry, TableOwner},
    TableType,
};

use super::StorageStrategy;

pub static SINGLE_QUOTED_STRING_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"'([^']*)'"#).unwrap());

#[derive(Debug)]
pub struct BruteForceStoragePolicy {
    owners: Option<Vec<TableOwner>>,
    blacklist: RefCell<HashSet<TableOwner>>,
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
            .filter_map(|capture| capture.get(1).as_ref().map(|s| s.as_str()))
            .filter_map(|capture| TableOwner::new(capture))
            .collect();

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
            .filter(|owner| !self.blacklist.borrow().contains(owner))
            .filter_map(|owner| TableEntry::new(table_type.clone(), owner.clone()))
            .collect::<Vec<_>>();

        let (tables, errors) = futures::future::join_all(
            entries
                .iter()
                .map(async move |entry| (entry.get_owner(), entry.fetch_data().await)),
        )
        .await
        .into_iter()
        .partition::<Vec<_>, _>(|result| result.1.is_ok());

        let errors = errors
            .into_iter()
            .map(|result| (result.0, result.1.unwrap_err()))
            .collect::<Vec<_>>();

        let (invalid_owners, errors) =
            errors
                .into_iter()
                .partition::<Vec<_>, _>(|(_, error)| match error {
                    DataFetchError::ApiError(api_error) => api_error.is_data_not_found(),
                    _ => false,
                });

        if !errors.is_empty() {
            anyhow::bail!("Failed to fetch data: {:?}", errors);
        }

        if !invalid_owners.is_empty() {
            for (owner, _) in invalid_owners {
                tracing::warn!("Invalid owner: {}", owner);
                self.blacklist.borrow_mut().insert(owner.clone());
            }
        }

        let tables = tables
            .into_iter()
            .map(|result| result.1.unwrap())
            .flatten()
            .collect();

        Ok(crate::sql::table::StorageTable::new(table_type, tables))
    }
}

impl BruteForceStoragePolicy {
    pub fn new() -> Self {
        Self {
            owners: Default::default(),
            blacklist: Default::default(),
        }
    }
}
