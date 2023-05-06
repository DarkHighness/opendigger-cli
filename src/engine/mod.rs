use crate::command::Commands;
use crate::sql::Storage;
use gluesql::prelude::Payload;
use once_cell::sync::Lazy;

#[derive(Debug)]
pub struct Engine {}

#[derive(Debug, thiserror::Error)]
pub enum EngineExecutionError {
    #[error("Failed to execute command {0}")]
    ApiError(#[from] crate::api::ApiError),
    #[error("Failed to execute command {0}")]
    IoError(#[from] std::io::Error),
    #[error("Failed to execute command {0}")]
    StorageError(#[from] crate::sql::StorageBuildError),
    #[error("Failed to execute command {0}")]
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

    pub async fn execute_command(&self, command: Commands) -> Result<(), EngineExecutionError> {
        tracing::debug!("Executing command: {:?}", command);

        match command {
            Commands::DownloadCommand(command) => {
                self.download_data(&command.name, command.metric, command.output_file)
                    .await?;
            }
            Commands::SqlQueryCommand(command) => {
                let (statements, entries) = (command.statements, command.entries);
                let storage = Storage::build_from_entries(&entries).await?;

                tracing::debug!("Storage: {:?}", storage);

                let mut engine = gluesql::prelude::Glue::new(storage);

                for statement in statements {
                    let payload = engine
                        .execute_stmt(&statement)
                        .map_err(|e| EngineExecutionError::EngineError(Box::new(e)))?;

                    tracing::debug!("Payload: {:?}", payload);

                    if let Payload::Select { labels, rows } = payload {
                        if let Some(output_file) = &command.output_file {
                            let output = crate::ui::render_csv(&labels, &rows);
                            tokio::fs::write(output_file, output).await?;
                        } else {
                            let output = crate::ui::render_table(&labels, &rows);
                            println!("{}", output);
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

pub static ENGINE: Lazy<Engine> = Lazy::new(Engine::new);
