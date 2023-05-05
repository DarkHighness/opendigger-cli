mod client;
mod error;
mod model;
mod types;

use crate::api::client::ApiClient;
use anyhow::Context;
use once_cell::sync::OnceCell;

pub use error::ApiError;
pub use types::{Metric, RepoMetric, Type, UserMetric};

pub static API: OnceCell<ApiClient> = OnceCell::new();

pub fn setup(base_url: &str) -> anyhow::Result<()> {
    let client =
        ApiClient::new(base_url).map_err(|_| anyhow::anyhow!("Failed to create API client"))?;

    API.set(client)
        .map_err(|_| anyhow::anyhow!("Failed to set API client"))?;

    Ok(())
}

pub fn get() -> &'static ApiClient {
    API.get().context("Failed to get API client").unwrap()
}
