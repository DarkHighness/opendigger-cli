#[derive(Debug)]
pub enum Commands {
    DownloadCommand(DownloadCommand),
}

#[derive(Debug)]
pub struct DownloadCommand {
    pub name: String,
    pub metric: crate::api::Metric,
    pub output_file: Option<String>,
}
