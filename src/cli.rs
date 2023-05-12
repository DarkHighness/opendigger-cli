use clap::error::ErrorKind;
use clap::{CommandFactory, Parser};
use std::str::FromStr;

use crate::api::Metric;
use crate::sql::StorageStrategyType;

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
        #[clap(short, long)]
        output_file: Option<String>,
        #[clap(name = "strategy", default_value_t = StorageStrategyType::BruteForce )]
        strategy: StorageStrategyType,
    },
}

fn unknown_metric_type_error(r#type: crate::api::Type, metric_type: &str) -> ! {
    let mut cmd = CLICommands::command();

    let available_types = match r#type {
        crate::api::Type::Repository => crate::api::RepositoryMetric::available_types(),
        crate::api::Type::User => crate::api::UserMetric::available_types(),
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

fn invalid_sql_query_error(sql: &str, reason: Option<&str>) -> ! {
    let mut cmd = CLICommands::command();

    if let Some(reason) = reason {
        cmd.error(
            ErrorKind::InvalidValue,
            format!("Invalid sql query: {sql}, reason: {reason}"),
        )
        .exit()
    } else {
        cmd.error(ErrorKind::InvalidValue, format!("Invalid sql query: {sql}"))
            .exit()
    }
}

pub async fn parse_command() -> crate::command::Commands {
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
                match crate::api::RepositoryMetric::from_str(r#type.as_str()) {
                    Ok(metric_type) => Metric::Repo(metric_type),
                    Err(_) => {
                        unknown_metric_type_error(crate::api::Type::Repository, r#type.as_str())
                    }
                }
            } else {
                match crate::api::UserMetric::from_str(r#type.as_str()) {
                    Ok(metric_type) => Metric::User(metric_type),
                    Err(_) => unknown_metric_type_error(crate::api::Type::User, r#type.as_str()),
                }
            };

            crate::command::Commands::new_download_command(name, metric, output_file)
        }
        Commands::SqlQuery {
            query,
            output_file,
            strategy,
        } => {
            let statements = gluesql::core::parse_sql::parse(&query);

            if let Err(err) = statements {
                invalid_sql_query_error(&query, Some(err.to_string().as_str()));
            }

            let (statements, errors): (Vec<_>, Vec<_>) = statements
                .unwrap()
                .into_iter()
                .map(|e| gluesql::core::translate::translate(&e))
                .partition(|e| e.is_ok());

            let (statements, errors): (Vec<_>, Vec<_>) = (
                statements.into_iter().map(|e| e.unwrap()).collect(),
                errors.into_iter().map(|e| e.unwrap_err()).collect(),
            );

            if !errors.is_empty() {
                let errors_in_string = errors
                    .iter()
                    .map(|e| format!("{:?}", e))
                    .collect::<Vec<_>>()
                    .join("\n");
                invalid_sql_query_error(&query, Some(&errors_in_string));
            }

            let strategy =
                crate::sql::create_strategy_instance(&strategy, &query, &statements).await;

            if let Err(err) = strategy {
                invalid_sql_query_error(query.as_str(), Some(err.to_string().as_str()));
            }

            let strategy = strategy.unwrap();

            crate::command::Commands::new_sql_query_command(strategy, statements, output_file)
        }
    }
}
