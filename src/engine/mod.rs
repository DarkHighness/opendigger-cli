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

    async fn download_repo_data(
        &self,
        repo_name: String,
        metric_type: crate::api::RepoMetricTypes,
        output_file: Option<String>,
    ) -> Result<(), EngineExecutionError> {
        let output_file = output_file.unwrap_or_else(|| {
            format!(
                "repo-{}-{}.json",
                repo_name.replace('/', "-"),
                metric_type.as_ref()
            )
        });

        let repo_data = crate::api::get()
            .repo_bytes(repo_name.as_str(), metric_type)
            .await?;

        tokio::fs::write(output_file, repo_data).await?;

        Ok(())
    }

    async fn download_user_data(
        &self,
        user_name: String,
        metric_type: crate::api::UserMetricTypes,
        output_file: Option<String>,
    ) -> Result<(), EngineExecutionError> {
        let output_file = output_file
            .unwrap_or_else(|| format!("user-{}-{}.json", user_name, metric_type.as_ref()));

        let user_data = crate::api::get()
            .user_bytes(user_name.as_str(), metric_type)
            .await?;

        tokio::fs::write(output_file, user_data).await?;

        Ok(())
    }

    pub async fn execute_command(&self, command: crate::command::Commands) {
        tracing::debug!("Executing command: {:?}", command);

        match command {
            Commands::DownloadRepoDataCommand(command) => {
                self.download_repo_data(
                    command.repo_name,
                    command.metric_type,
                    command.output_file,
                )
                .await
                .unwrap();
            }
            Commands::DownloadUserDataCommand(command) => {
                self.download_user_data(
                    command.user_name,
                    command.metric_type,
                    command.output_file,
                )
                .await
                .unwrap();
            }
        }
    }
}

pub static ENGINE: Lazy<Engine> = Lazy::new(Engine::new);
