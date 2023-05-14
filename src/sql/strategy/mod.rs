use std::fmt::Debug;
use std::fmt::Display;

use async_trait::async_trait;
use gluesql::core::ast;
use gluesql::core::data::Schema;

use super::TableType;

mod analyze;
mod brute_force;

#[async_trait(? Send)]
pub trait StorageStrategy: Debug {
    async fn analyze_query(
        &mut self,
        sql: &str,
        statements: &[ast::Statement],
    ) -> anyhow::Result<()>;

    async fn fetch_all_schemas(&self) -> anyhow::Result<Vec<Schema>> {
        Ok(super::ALL_SCHEMAS.clone())
    }

    async fn fetch_table(
        &self,
        table_type: TableType,
    ) -> anyhow::Result<super::table::StorageTable>;
}

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
) -> anyhow::Result<Box<dyn StorageStrategy>> {
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
