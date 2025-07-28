use std::{fs::File, io::Write, path::Path, time::Duration};

use anyhow::Result;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub url: String,
    pub enable_logging: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_connections: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_timeout: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_timeout: Option<u64>,
}

impl Config {
    pub fn default_with_url(url: &str) -> Self {
        Self {
            url: url.to_string(),
            ..Default::default()
        }
    }

    pub fn load_from_file(path: impl AsRef<Path>) -> Result<Self> {
        let config = toml::from_str(&std::fs::read_to_string(path)?)?;
        Ok(config)
    }
    pub fn create_file(path: impl AsRef<Path>) -> Result<()> {
        let config = Self {
            url: String::new(),
            enable_logging: false,
            min_connections: Some(5),
            max_connections: Some(10),
            connect_timeout: Some(1000),
            idle_timeout: Some(1000),
        };
        let content = toml::to_string_pretty(&config)?;
        let mut file = File::create(path)?;
        file.write_all(content.as_bytes())?;
        file.sync_all()?;
        Ok(())
    }
}

pub async fn db_connect(config: &Config) -> Result<DatabaseConnection> {
    tracing::debug!("Connecting to database: {:#?}", config);
    let mut opt = ConnectOptions::new(&config.url);

    if let Some(max_connections) = config.max_connections {
        opt.max_connections(max_connections);
    }
    if let Some(min_connections) = config.min_connections {
        opt.min_connections(min_connections);
    }
    if let Some(connect_timeout) = config.connect_timeout {
        opt.connect_timeout(Duration::from_millis(connect_timeout));
    }
    if let Some(idle_timeout) = config.idle_timeout {
        opt.idle_timeout(Duration::from_millis(idle_timeout));
    }

    opt.sqlx_logging(config.enable_logging);

    let db = Database::connect(opt).await?;
    tracing::info!("Database connected successfully");
    Ok(db)
}
