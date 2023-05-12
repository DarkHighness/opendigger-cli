use crate::command::Commands;
use crate::sql::Storage;
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
                    if let Some(output_file) = output_file.as_ref() {
                        let output = crate::ui::render_csv_row(&labels, &rows);
                        tokio::fs::write(output_file, output).await?;
                    } else {
                        let output = crate::ui::render_table_row(&labels, &rows);
                        println!("{}", output);
                    }
                }
                Payload::ShowColumns(columns) => {
                    let output = crate::ui::render_table(
                        &["column".to_string(), "type".to_string()],
                        &columns
                            .into_iter()
                            .map(|(name, typ)| vec![name, typ.to_string()])
                            .collect::<Vec<_>>(),
                    );
                    println!("{}", output);
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
                self.execute_sql_query(command.statements, command.strategy, command.output_file)
                    .await?;
            }
        }

        Ok(())
    }
}

pub static ENGINE: Lazy<Engine> = Lazy::new(Engine::new);
