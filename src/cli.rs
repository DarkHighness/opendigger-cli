use clap::error::ErrorKind;
use clap::{CommandFactory, Parser};
use std::str::FromStr;

use crate::api::Metric;

#[derive(Debug, clap::Parser)]
#[command(name = "opendigger-cli")]
#[command(author, version, about)]
pub struct CLICommands {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, clap::Subcommand)]
pub enum Commands {
    #[clap(about = "Download data from the API")]
    Download {
        #[clap(name = "repo name|user name")]
        name: String,
        #[clap(name = "type")]
        r#type: String,
        #[clap(short, long)]
        output_file: Option<String>,
    },
    #[clap(about = "Query data with sql")]
    #[clap(name = "sql")]
    SqlQuery {
        #[clap(name = "query")]
        query: String,
    },
}

fn unknown_metric_type_error(r#type: crate::api::Target, metric_type: &str) -> ! {
    let mut cmd = CLICommands::command();

    let available_types = match r#type {
        crate::api::Target::Repo => crate::api::RepoMetric::available_types(),
        crate::api::Target::User => crate::api::UserMetric::available_types(),
    };
    let available_types = available_types.join(", ");

    cmd.error(
        ErrorKind::InvalidValue,
        format!(
            "Unknown {} metric type: {}\n\tAvailable types: {}",
            r#type.as_ref(),
            metric_type,
            available_types
        ),
    )
    .exit()
}

fn invalid_sql_query_error(sql: &str) -> ! {
    let mut cmd = CLICommands::command();

    cmd.error(ErrorKind::InvalidValue, format!("Invalid sql query: {sql}"))
        .exit()
}

pub fn parse_command() -> crate::command::Commands {
    let cli_args = CLICommands::parse();
    let cli_command = cli_args.command;

    match cli_command {
        Commands::Download {
            name,
            r#type,
            output_file,
        } => {
            let is_repo_name = name.contains('/');

            let metric = if is_repo_name {
                match crate::api::RepoMetric::from_str(r#type.as_str()) {
                    Ok(metric_type) => Metric::Repo(metric_type),
                    Err(_) => unknown_metric_type_error(crate::api::Target::Repo, r#type.as_str()),
                }
            } else {
                match crate::api::UserMetric::from_str(r#type.as_str()) {
                    Ok(metric_type) => Metric::User(metric_type),
                    Err(_) => unknown_metric_type_error(crate::api::Target::User, r#type.as_str()),
                }
            };

            crate::command::Commands::new_download_command(name, metric, output_file)
        }
        Commands::SqlQuery { query } => {
            let dialect = sqlparser::dialect::GenericDialect;

            match sqlparser::parser::Parser::parse_sql(&dialect, &query) {
                Ok(statements) => {
                    let before_len = statements.len();
                    let queries: Vec<sqlparser::ast::Query> = statements
                        .into_iter()
                        .filter_map(|e| {
                            if let sqlparser::ast::Statement::Query(query) = e {
                                let query = Box::<sqlparser::ast::Query>::into_inner(query);
                                Some(query)
                            } else {
                                None
                            }
                        })
                        .collect();
                    let after_len = queries.len();

                    if before_len != after_len || after_len == 0 {
                        tracing::warn!("Only select statement is supported in sql query");
                    }

                    crate::command::Commands::new_sql_query_command(queries)
                }
                Err(_) => invalid_sql_query_error(query.as_str()),
            }
        }
    }
}
