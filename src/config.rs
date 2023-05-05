use anyhow::Context;
use config::{Config, Environment, File};
use once_cell::sync::Lazy;
use serde::Deserialize;

pub static DEFAULT_BASE_URL: &str = "https://oss.x-lab.info/open_digger/github";
pub static OPEN_DIGGER_CLI_CONFIG: Lazy<OpenDiggerCLIConfig> =
    Lazy::new(OpenDiggerCLIConfig::load_config);

#[derive(Debug, Deserialize)]
pub struct OpenDiggerCLIConfig {
    base_url: Option<String>,
}

impl OpenDiggerCLIConfig {
    fn load_config() -> OpenDiggerCLIConfig {
        let config = Config::builder()
            .add_source(File::with_name("config").required(false))
            .add_source(Environment::with_prefix("OPEN_DIGGER_CLI"))
            .build()
            .context("Building config")
            .unwrap();

        config
            .try_deserialize()
            .context("Deserializing config")
            .unwrap()
    }

    pub fn base_url(&self) -> &str {
        self.base_url.as_deref().unwrap_or(DEFAULT_BASE_URL)
    }
}
