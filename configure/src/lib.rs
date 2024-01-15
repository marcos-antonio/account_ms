use std::path::{Path, PathBuf};

use config::ConfigError;
use once_cell::sync::Lazy;
use postgres::PostgresConfig;
use profile::Profile;
use rabbit_mq::RabbitMQConfig;
use serde::Deserialize;
use server::ServerConfig;

pub mod postgres;
pub mod profile;
pub mod rabbit_mq;
pub mod server;

pub static CONFIG: Lazy<AppConfig> = Lazy::new(|| AppConfig::read().unwrap());

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub profile: Profile,
    pub server: ServerConfig,
    pub postgres: PostgresConfig,
    pub rabbit_mq: RabbitMQConfig,
}

impl AppConfig {
    pub fn read() -> Result<Self, config::ConfigError> {
        let config_dir = root_dir("settings").map_err(|e| ConfigError::Message(e.to_string()))?;
        let profile: Profile = std::env::var("APP_PROFILE")
            .unwrap_or_else(|_| "dev".into())
            .try_into()
            .map_err(ConfigError::Message)?;
        let profile_name = format!("{profile}.toml");

        let config = config::Config::builder()
            .add_source(config::File::from(config_dir.join("base.toml")))
            .add_source(config::File::from(config_dir.join(profile_name)))
            .build()?;

        config.try_deserialize()
    }
}

fn root_dir<P: AsRef<Path>>(path: P) -> std::io::Result<PathBuf> {
    Ok(project_root::get_project_root()
        .or_else(|_| std::env::current_dir())?
        .join(path))
}
