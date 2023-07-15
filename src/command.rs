use gluesql::core::ast;

#[derive(Debug)]
pub enum Commands {
    Download(DownloadCommand),
    SqlQuery(SqlQueryCommand),
    CypherQuery(CypherQueryCommand),
    Report(ReportCommand),
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
    pub ui_mode: crate::ui::UIMode,
}

#[derive(Debug)]
pub struct CypherQueryCommand {
    pub statements: String,
    pub output_file: Option<String>,
    pub ui_mode: crate::ui::UIMode,
}

#[derive(Debug)]
pub struct ReportCommand {
    pub owner: String,
    pub time: Option<String>,
}

impl Commands {
    pub fn new_download_command(
        name: String,
        metric: crate::api::Metric,
        output_file: Option<String>,
    ) -> Commands {
        Self::Download(DownloadCommand {
            name,
            metric,
            output_file,
        })
    }

    pub fn new_sql_query_command(
        strategy: Box<dyn crate::sql::StorageStrategy>,
        statements: Vec<ast::Statement>,
        output_file: Option<String>,
        ui_mode: crate::ui::UIMode,
    ) -> Commands {
        Self::SqlQuery(SqlQueryCommand {
            strategy,
            statements,
            output_file,
            ui_mode,
        })
    }

    pub fn new_cypher_query_command(
        statements: String,
        output_file: Option<String>,
        ui_mode: crate::ui::UIMode,
    ) -> Commands {
        Self::CypherQuery(CypherQueryCommand {
            statements,
            output_file,
            ui_mode,
        })
    }

    pub fn new_report_command(owner: String, time: Option<String>) -> Commands {
        Self::Report(ReportCommand { owner, time })
    }
}
