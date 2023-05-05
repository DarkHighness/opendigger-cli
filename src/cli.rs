use clap::error::ErrorKind;
use clap::{CommandFactory, Parser};
use std::str::FromStr;

use crate::api::MetricType;

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
        #[clap(name = "repo name|user Name")]
        name: String,
        #[clap(name = "type")]
        r#type: String,
        #[clap(short, long)]
        output_file: Option<String>,
    },
}

fn unknown_metric_type_error(r#type: crate::api::TargetType, metric_type: &str) -> ! {
    let mut cmd = CLICommands::command();

    let available_types = match r#type {
        crate::api::TargetType::Repo => crate::api::RepoMetricTypes::available_types(),
        crate::api::TargetType::User => crate::api::UserMetricTypes::available_types(),
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

            if is_repo_name {
                match crate::api::RepoMetricTypes::from_str(r#type.as_str()) {
                    Ok(metric_type) => crate::command::Commands::DownloadRepoDataCommand(
                        crate::command::DownloadRepoDataCommand {
                            repo_name: name,
                            metric_type,
                            output_file,
                        },
                    ),
                    Err(_) => {
                        unknown_metric_type_error(crate::api::TargetType::Repo, r#type.as_str())
                    }
                }
            } else {
                match crate::api::UserMetricTypes::from_str(r#type.as_str()) {
                    Ok(metric_type) => crate::command::Commands::DownloadUserDataCommand(
                        crate::command::DownloadUserDataCommand {
                            user_name: name,
                            metric_type,
                            output_file,
                        },
                    ),
                    Err(_) => {
                        unknown_metric_type_error(crate::api::TargetType::User, r#type.as_str())
                    }
                }
            }
        }
    }
}
