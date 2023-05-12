use async_trait::async_trait;
use gluesql::core::ast;

use crate::sql::{table::TableEntry, TableType};

use super::StorageStrategy;

mod analyzer;

#[derive(Debug)]
pub struct AnalyzeBasedStorageStrategy {
    entries: Option<Vec<TableEntry>>,
}

#[async_trait(? Send)]
impl StorageStrategy for AnalyzeBasedStorageStrategy {
    async fn analyze_query(
        &mut self,
        _query: &str,
        statements: &[ast::Statement],
    ) -> anyhow::Result<()> {
        let analyzer = analyzer::Analyzer::new();
        let output = analyzer
            .analyze_statements(&statements)
            .map_err(|err| Box::new(err))?;

        self.entries = Some(output.tables);

        Ok(())
    }

    async fn fetch_table(
        &self,
        table_type: TableType,
    ) -> anyhow::Result<crate::sql::table::StorageTable> {
        let entries = self.entries.as_ref().unwrap();

        let (tables, errors) = futures::future::join_all(
            entries
                .iter()
                .filter(|entry| entry.r#type == table_type)
                .map(|entry| entry.fetch_data()),
        )
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

impl AnalyzeBasedStorageStrategy {
    pub fn new() -> Self {
        Self {
            entries: Default::default(),
        }
    }
}
