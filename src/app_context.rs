use anyhow::Context;

use crate::config::Config;

pub struct AppContext {
    pub config: Config, // logger, global settings, etc.
}

impl AppContext {
    pub fn default_init() -> anyhow::Result<Self> {
        Ok(Self {
            config: Config::default_init().context("Initializing config")?,
        })
    }

    pub fn custom(config: Config) -> AppContext {
        AppContext { config }
    }
}
