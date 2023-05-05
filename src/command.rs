#[derive(Debug)]
pub enum Commands {
    DownloadCommand(DownloadCommand),
    SqlQueryCommand(SqlQueryCommand),
}

#[derive(Debug)]
pub struct DownloadCommand {
    pub name: String,
    pub metric: crate::api::Metric,
    pub output_file: Option<String>,
}

#[derive(Debug)]
pub struct SqlQueryCommand {
    pub queries: Vec<sqlparser::ast::Query>,
}

impl Commands {
    pub fn new_download_command(
        name: String,
        metric: crate::api::Metric,
        output_file: Option<String>,
    ) -> Commands {
        Self::DownloadCommand(DownloadCommand {
            name,
            metric,
            output_file,
        })
    }

    pub fn new_sql_query_command(queries: Vec<sqlparser::ast::Query>) -> Commands {
        Self::SqlQueryCommand(SqlQueryCommand { queries })
    }
}
