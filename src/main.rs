use anyhow::Context;
use std::collections::btree_map::BTreeMap;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod api;
mod config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_timer(tracing_subscriber::fmt::time::LocalTime::rfc_3339()),
        )
        .init();

    color_eyre::install().map_err(|_| anyhow::anyhow!("Failed to install color_eyre"))?;

    let base_url = config::OPEN_DIGGER_CLI_CONFIG
        .base_url();

    api::setup(base_url).context("Setting up Api client")?;

    let repo_name = "X-lab2017/open-digger";
    let user_name = "frank-zsy";

    let api = api::get();

    let openrank = api
        .repos::<BTreeMap<String, f64>>(repo_name, api::RepoMetricTypes::OpenRank)
        .await?;

    for (k, v) in openrank.iter() {
        println!("{}: {}", k, v);
    }

    let openrank = api
        .users::<BTreeMap<String, f64>>(user_name, api::UserMetricTypes::OpenRank)
        .await?;

    for (k, v) in openrank.iter() {
        println!("{}: {}", k, v);
    }

    Ok(())
}
