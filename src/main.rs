#![feature(let_chains)]
#![feature(box_into_inner)]
#![feature(async_closure)]

use anyhow::Context;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod api;
mod cli;
mod command;
mod config;
mod engine;
mod sql;
mod ui;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(
            tracing_subscriber::fmt::layer()
                .with_timer(tracing_subscriber::fmt::time::LocalTime::rfc_3339()),
        )
        .init();

    color_eyre::install().map_err(|_| anyhow::anyhow!("Failed to install color_eyre"))?;

    let base_url = config::OPEN_DIGGER_CLI_CONFIG.base_url();

    api::setup(base_url).context("Setting up Api client")?;

    let command = cli::parse_command();

    engine::ENGINE
        .execute_command(command)
        .await
        .map_err(|err| anyhow::anyhow!("Failed to execute command: {:?}", err))?;

    Ok(())
}
