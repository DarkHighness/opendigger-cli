#[derive(Debug)]
pub enum Commands {
    DownloadRepoDataCommand(DownloadRepoDataCommand),
    DownloadUserDataCommand(DownloadUserDataCommand),
}

#[derive(Debug)]
pub struct DownloadRepoDataCommand {
    pub repo_name: String,
    pub metric_type: crate::api::RepoMetricTypes,
    pub output_file: Option<String>,
}

#[derive(Debug)]
pub struct DownloadUserDataCommand {
    pub user_name: String,
    pub metric_type: crate::api::UserMetricTypes,
    pub output_file: Option<String>,
}
