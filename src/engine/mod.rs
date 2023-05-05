use crate::command::Commands;
use once_cell::sync::Lazy;

#[derive(Debug)]
pub struct Engine {}

#[derive(Debug, thiserror::Error)]
pub enum EngineExecutionError {
    #[error("Failed to execute command {0}")]
    ApiError(#[from] crate::api::ApiError),
    #[error("Failed to execute command {0}")]
    IoError(#[from] std::io::Error),
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

    pub async fn execute_command(&self, command: Commands) {
        tracing::debug!("Executing command: {:?}", command);

        match command {
            Commands::DownloadCommand(command) => {
                self.download_data(&command.name, command.metric, command.output_file)
                    .await
                    .unwrap();
            }
        }
    }
}

pub static ENGINE: Lazy<Engine> = Lazy::new(Engine::new);
