use crate::command::Commands;
use crate::sql::Storage;
use crate::ui::{TableUI, UIMode};
use anyhow::Context;
use gluesql::core::ast;
use gluesql::prelude::{Payload, PayloadVariable};
use once_cell::sync::Lazy;

pub use util::{execute_sql_queries, execute_sql_query};
pub use value::{translate_from_gluesql_value, Value};

mod util;
mod value;

#[derive(Debug)]
pub struct Engine {}

#[derive(Debug, thiserror::Error)]
pub enum EngineExecutionError {
    #[error(transparent)]
    Api(#[from] crate::api::ApiError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Storage(#[from] crate::sql::StorageError),
    #[error(transparent)]
    Engine(#[from] Box<dyn std::error::Error + Send + Sync>),
    #[error(transparent)]
    UI(#[from] crate::ui::UIError),
    #[error(transparent)]
    Report(#[from] crate::report::ReportError),
}

impl Engine {
    fn new() -> Self {
        Engine {}
    }

    async fn download_data(
        &self,
        name: &str,
        metric: crate::api::Metric,
        output_file: Option<String>,
    ) -> Result<(), EngineExecutionError> {
        let output_file = output_file
            .unwrap_or_else(|| format!("repo-{}-{}.json", name.replace('/', "-"), metric.as_ref()));

        let repo_data = crate::api::get().bytes(name, metric).await?;

        tokio::fs::write(output_file, repo_data).await?;

        Ok(())
    }

    async fn execute_sql_query(
        &self,
        statements: Vec<ast::Statement>,
        strategy: Box<dyn crate::sql::StorageStrategy>,
        output_file: Option<String>,
        ui_mode: UIMode,
    ) -> Result<(), EngineExecutionError> {
        let storage = Storage::build_from_strategy(strategy).await?;

        tracing::debug!("Storage: {:?}", storage);

        let mut engine = gluesql::prelude::Glue::new(storage);

        for statement in statements {
            let payload = engine
                .execute_stmt(&statement)
                .map_err(|e| EngineExecutionError::Engine(Box::new(e)))?;

            tracing::debug!("Payload: {:?}", payload);

            match payload {
                Payload::Select { labels, rows } => {
                    let rows = rows
                        .into_iter()
                        .map(|row| {
                            row.into_iter()
                                .map(|value| {
                                    translate_from_gluesql_value(value)
                                        .context("Failed to translate GlueSQL value to Value")
                                        .unwrap()
                                })
                                .collect::<Vec<_>>()
                        })
                        .collect::<Vec<_>>();

                    if let Some(output_file) = output_file.as_ref() {
                        let rows = rows
                            .iter()
                            .map(|row| {
                                row.iter()
                                    .map(|value| value.to_string())
                                    .collect::<Vec<_>>()
                            })
                            .collect::<Vec<_>>();
                        let output = crate::ui::render_csv(&labels, &rows);

                        tokio::fs::write(output_file, output).await?;

                        tracing::info!("Output written to {}", output_file);
                    }

                    let time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
                    let title = format!(" Query Result: {} ", time);

                    TableUI::new(ui_mode, title, labels, rows)?.render()?;
                }
                Payload::ShowColumns(columns) => {
                    let rows = columns
                        .into_iter()
                        .map(|(name, typ)| vec![name.into(), typ.to_string().into()])
                        .collect::<Vec<_>>();

                    TableUI::new(
                        ui_mode,
                        "Columns".to_string(),
                        vec!["Column".to_string(), "Type".to_string()],
                        rows,
                    )?
                    .render()?;
                }
                Payload::ShowVariable(payload) => {
                    if let PayloadVariable::Tables(tables) = payload {
                        let rows = tables
                            .into_iter()
                            .map(|table| vec![table.into()])
                            .collect::<Vec<_>>();

                        TableUI::new(ui_mode, "Tables".to_string(), vec!["name".into()], rows)?
                            .render()?;
                    } else {
                        tracing::error!("Invalid payload: {:?}", payload);
                    }
                }
                _ => {
                    tracing::error!("Invalid payload: {:?}", payload);
                }
            }
        }

        Ok(())
    }

    pub async fn execute_command(&self, command: Commands) -> Result<(), EngineExecutionError> {
        tracing::debug!("Executing command: {:?}", command);

        match command {
            Commands::Download(command) => {
                self.download_data(&command.name, command.metric, command.output_file)
                    .await?;
            }
            Commands::SqlQuery(command) => {
                self.execute_sql_query(
                    command.statements,
                    command.strategy,
                    command.output_file,
                    command.ui_mode,
                )
                .await?;
            }
            Commands::Report(command) => {
                let owner = command.owner;
                if owner.contains('/') {
                    let mut report = crate::report::RepoOverview::new(owner);
                    report.generate_report().await?;

                    //crate::ui::RepoOverviewUI::new(report).run()?;
                }
            }
        }

        Ok(())
    }
}

pub static ENGINE: Lazy<Engine> = Lazy::new(Engine::new);
