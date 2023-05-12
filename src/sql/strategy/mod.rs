use std::fmt::Debug;
use std::fmt::Display;

use async_trait::async_trait;
use gluesql::core::ast;
use gluesql::core::data::Schema;

use crate::sql::table::ACTIVITY_TABLE_SCHEMA;
use crate::sql::table::ATTENTION_TABLE_SCHEMA;
use crate::sql::table::OPENRANK_TABLE_SCHEMA;

use super::TableType;

mod analyze;
mod brute_force;

#[async_trait(? Send)]
pub trait StorageStrategy: Debug {
    async fn analyze_query(
        &mut self,
        sql: &str,
        statements: &[ast::Statement],
    ) -> Result<(), StorageStrategyError>;

    async fn fetch_all_schemas(&self) -> Result<Vec<Schema>, StorageStrategyError> {
        Ok(vec![
            ACTIVITY_TABLE_SCHEMA.clone(),
            ATTENTION_TABLE_SCHEMA.clone(),
            OPENRANK_TABLE_SCHEMA.clone(),
        ])
    }

    async fn fetch_table(
        &self,
        table_type: TableType,
    ) -> Result<super::table::StorageTable, StorageStrategyError>;
}

pub type StorageStrategyError = Box<dyn std::error::Error>;

#[derive(
    Debug, Clone, Copy, strum::EnumString, strum::AsRefStr, strum::IntoStaticStr, serde::Deserialize,
)]
pub enum StorageStrategyType {
    #[serde(rename = "BruteForce")]
    #[strum(serialize = "BruteForce")]
    BruteForce,
    #[serde(rename = "AnalyzeBased")]
    #[strum(serialize = "AnalyzeBased")]
    AnalyzeBased,
}

impl Display for StorageStrategyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

pub async fn create_strategy_instance(
    strategy: &StorageStrategyType,
    sql: &str,
    statements: &[ast::Statement],
) -> Result<Box<dyn StorageStrategy>, StorageStrategyError> {
    let mut instance = match strategy {
        StorageStrategyType::BruteForce => {
            Box::new(brute_force::BruteForceStoragePolicy::new()) as Box<dyn StorageStrategy>
        }
        StorageStrategyType::AnalyzeBased => {
            Box::new(analyze::AnalyzeBasedStorageStrategy::new()) as Box<dyn StorageStrategy>
        }
    };

    instance.analyze_query(sql, statements).await?;

    Ok(instance)
}
