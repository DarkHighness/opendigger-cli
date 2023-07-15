use gluesql::prelude::{Payload, Value};

use crate::sql::Storage;

#[derive(Debug)]
pub struct Output {
    pub items: Vec<(String, Value)>,
}

pub async fn execute_sql_query(query: &str) -> anyhow::Result<Output> {
    let statements = gluesql::core::parse_sql::parse(query)?;

    if statements.len() != 1 {
        anyhow::bail!("Only one statement is supported");
    }

    let statements = statements
        .into_iter()
        .map(|e| gluesql::core::translate::translate(&e))
        .collect::<Result<Vec<_>, _>>()?;

    let strategy = crate::sql::create_strategy_instance(
        &crate::sql::StorageStrategyType::BruteForce,
        query,
        &statements,
    )
    .await?;

    let storage = Storage::build_from_strategy(strategy).await?;
    let mut engine = gluesql::prelude::Glue::new(storage);

    let payload = engine.execute_stmt(&statements[0])?;

    match payload {
        Payload::Select { labels, rows } => {
            let items = rows
                .into_iter()
                .flat_map(|row| {
                    labels
                        .iter()
                        .zip(row.into_iter())
                        .map(|(label, value)| (label.to_string(), value))
                })
                .collect::<Vec<_>>();

            let output = Output { items };

            Ok(output)
        }
        _ => {
            anyhow::bail!("Unsupported payload: {:?}", payload);
        }
    }
}

impl Output {
    pub fn item(&self) -> &(String, Value) {
        &self.items[0]
    }

    pub fn first(&self) -> &(String, Value) {
        &self.items[0]
    }

    pub fn last(&self) -> &(String, Value) {
        &self.items[self.items.len() - 1]
    }
}

pub async fn execute_sql_queries(query: &str) -> anyhow::Result<Vec<Output>> {
    let statements = gluesql::core::parse_sql::parse(query)?;

    let statements = statements
        .into_iter()
        .map(|e| gluesql::core::translate::translate(&e))
        .collect::<Result<Vec<_>, _>>()?;

    let strategy = crate::sql::create_strategy_instance(
        &crate::sql::StorageStrategyType::BruteForce,
        query,
        &statements,
    )
    .await?;

    let storage = Storage::build_from_strategy(strategy).await?;
    let mut engine = gluesql::prelude::Glue::new(storage);

    let outputs = statements
        .into_iter()
        .map(|statement| engine.execute_stmt(&statement))
        .filter_map(|payload| match payload {
            Ok(payload) => match payload {
                Payload::Select { labels, rows } => {
                    let items = rows
                        .into_iter()
                        .flat_map(|row| {
                            labels
                                .iter()
                                .zip(row.into_iter())
                                .map(|(label, value)| (label.to_string(), value))
                        })
                        .collect::<Vec<_>>();

                    let output = Output { items };

                    Some(output)
                }
                _ => {
                    tracing::warn!("Unsupported payload: {:?}", payload);
                    None
                }
            },
            Err(err) => {
                tracing::error!("Error: {:?}", err);
                None
            }
        })
        .collect::<Vec<_>>();

    Ok(outputs)
}


