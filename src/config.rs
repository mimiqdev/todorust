use crate::error::{TodoError, Result};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub api_token: String,
}

pub fn load_config() -> Result<Config> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| TodoError::Config("Cannot find config directory".to_string()))?
        .join("todoirust");

    let config_path = config_dir.join("config.toml");

    let content = fs::read_to_string(&config_path)
        .map_err(|e| TodoError::Config(format!("Cannot read config: {}", e)))?;

    let config: Config = toml::from_str(&content)
        .map_err(|e| TodoError::Config(format!("Cannot parse config: {}", e)))?;

    Ok(config)
}
