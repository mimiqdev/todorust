//! # Configuration
//!
//! Manages application configuration, including API token storage and retrieval.
//! Configuration is stored in the platform-specific config directory.

use crate::error::{Result, TodoError};
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

    if !config_path.exists() {
        return Err(TodoError::ConfigNotFound);
    }

    let content = fs::read_to_string(&config_path)
        .map_err(|e| TodoError::Config(format!("Cannot read config: {}", e)))?;

    let config: Config = toml::from_str(&content)?;

    Ok(config)
}

pub fn init_config(api_token: &str) -> Result<()> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| TodoError::Config("Cannot find config directory".to_string()))?
        .join("todoirust");

    fs::create_dir_all(&config_dir)
        .map_err(|e| TodoError::Config(format!("Cannot create config directory: {}", e)))?;

    let config = Config {
        api_token: api_token.to_string(),
    };

    let toml_str = toml::to_string(&config)
        .map_err(|e| TodoError::Config(format!("Cannot serialize config: {}", e)))?;

    let config_path = config_dir.join("config.toml");
    fs::write(&config_path, toml_str)
        .map_err(|e| TodoError::Config(format!("Cannot write config: {}", e)))?;

    println!("Config saved to {}", config_path.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_parsing() {
        let toml_str = r#"
            api_token = "test_token_123"
        "#;

        let config: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(config.api_token, "test_token_123");
    }
}
