use std::fmt::Display;
use anyhow::{bail, Result};
use serde::Deserialize;
use std::path::PathBuf;
use crate::config::application::app::App;
use crate::config::application::database::Database;
use crate::config::application::server::Server;
use crate::config::application::logging::Logging;
use crate::config::application::redis::Redis;

#[derive(Debug, Default, Clone, Deserialize)]
pub struct ApplicationConfig {
    pub app: App,
    pub server: Server,
    pub logging: Option<Logging>,
    pub database: Database,
    pub redis: Redis,
}

impl ApplicationConfig {
    pub fn init(file_path: PathBuf) -> Result<Self> {
        if !file_path.exists() {
            bail!("config file not exists");
        }
        let file_path: &str = file_path.to_str().unwrap();
        let config = config::Config::builder()
            .add_source(config::File::with_name(file_path))
            .build()?;
        Ok(config.try_deserialize::<ApplicationConfig>()?)
    }
}

impl Display for ApplicationConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n", self.server)?;
        if let Some(logging) = &self.logging {
            write!(f, "{}\n", logging)?;
        }
        write!(f, "{}\n", self.database)
    }
}
