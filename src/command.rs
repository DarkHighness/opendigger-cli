use gluesql::core::ast;

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
    pub strategy: Box<dyn crate::sql::StorageStrategy>,
    pub statements: Vec<ast::Statement>,
    pub output_file: Option<String>,
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

    pub fn new_sql_query_command(
        strategy: Box<dyn crate::sql::StorageStrategy>,
        statements: Vec<ast::Statement>,
        output_file: Option<String>,
    ) -> Commands {
        Self::SqlQueryCommand(SqlQueryCommand {
            strategy,
            statements,
            output_file,
        })
    }
}
