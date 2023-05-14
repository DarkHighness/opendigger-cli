use crate::command::Commands;
use crate::sql::Storage;
use crate::ui::{TableUI, UIMode};
use gluesql::core::ast;
use gluesql::prelude::Payload;
use once_cell::sync::Lazy;

#[derive(Debug)]
pub struct Engine {}

#[derive(Debug, thiserror::Error)]
pub enum EngineExecutionError {
    #[error(transparent)]
    ApiError(#[from] crate::api::ApiError),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    StorageError(#[from] crate::sql::StorageError),
    #[error(transparent)]
    EngineError(#[from] Box<dyn std::error::Error + Send + Sync>),
    #[error(transparent)]
    UIError(#[from] crate::ui::UIError),
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
                .map_err(|e| EngineExecutionError::EngineError(Box::new(e)))?;

            tracing::debug!("Payload: {:?}", payload);

            match payload {
                Payload::Select { labels, rows } => {
                    let rows = crate::ui::render_rows(&rows);

                    if let Some(output_file) = output_file.as_ref() {
                        let output = crate::ui::render_csv(&labels, &rows);
                        tokio::fs::write(output_file, output).await?;

                        tracing::info!("Output written to {}", output_file);
                    }

                    let time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
                    let title = format!(" Query Result: {} ", time);

                    TableUI::new(ui_mode, title, labels, rows)?.run()?;
                }
                Payload::ShowColumns(columns) => {
                    let rows = columns
                        .into_iter()
                        .map(|(name, typ)| vec![name, typ.to_string()])
                        .collect::<Vec<_>>();

                    TableUI::new(
                        ui_mode,
                        "Columns".to_string(),
                        vec!["Column".to_string(), "Type".to_string()],
                        rows,
                    )?
                    .run()?;
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
            Commands::DownloadCommand(command) => {
                self.download_data(&command.name, command.metric, command.output_file)
                    .await?;
            }
            Commands::SqlQueryCommand(command) => {
                self.execute_sql_query(
                    command.statements,
                    command.strategy,
                    command.output_file,
                    command.ui_mode,
                )
                .await?;
            }
            Commands::ReportCommand(_command) => {
                unimplemented!("Report command not implemented yet")
            }
        }

        Ok(())
    }
}

pub static ENGINE: Lazy<Engine> = Lazy::new(Engine::new);
