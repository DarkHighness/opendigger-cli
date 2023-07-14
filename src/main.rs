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
mod report;
mod sql;
mod ui;
mod networkgraph;

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

    //networkgraph::table::generic_network::fetch_network_data2().await;
    networkgraph::analyze::test().await;

/* 
    let command = cli::parse_command().await;

    engine::ENGINE
        .execute_command(command)
        .await
        .map_err(|err| anyhow::anyhow!("Failed to execute command: {:?}", err))?;
*/
    Ok(())
   

}

