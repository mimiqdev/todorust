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
    use tempfile::tempdir;

    #[test]
    fn test_config_parsing() {
        let toml_str = r#"
            api_token = "test_token_123"
        "#;

        let config: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(config.api_token, "test_token_123");
    }

    #[test]
    fn test_config_with_empty_token() {
        let toml_str = r#"
            api_token = ""
        "#;

        let config: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(config.api_token, "");
    }

    #[test]
    fn test_config_serialization() {
        let config = Config {
            api_token: "my_secret_token".to_string(),
        };

        let toml_str = toml::to_string(&config).unwrap();
        assert!(toml_str.contains("api_token"));
        assert!(toml_str.contains("my_secret_token"));
    }

    #[test]
    fn test_config_deserialization_roundtrip() {
        let original = r#"
            api_token = "roundtrip_token_456"
        "#;

        let config: Config = toml::from_str(original).unwrap();
        let serialized = toml::to_string(&config).unwrap();
        let decoded: Config = toml::from_str(&serialized).unwrap();

        assert_eq!(config.api_token, decoded.api_token);
    }

    #[test]
    fn test_config_debug_format() {
        let config = Config {
            api_token: "secret".to_string(),
        };
        let debug_format = format!("{:?}", config);
        assert!(debug_format.contains("Config"));
        assert!(debug_format.contains("secret"));
    }

    #[test]
    fn test_config_with_special_characters() {
        let toml_str = r#"
            api_token = "token_with_special_!@#$%^&*()chars"
        "#;

        let config: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(config.api_token, "token_with_special_!@#$%^&*()chars");
    }

    #[test]
    fn test_config_serialization_format() {
        let config = Config {
            api_token: "test_token".to_string(),
        };
        let toml_str = toml::to_string(&config).unwrap();
        // TOML should have the format: api_token = "value"
        assert!(toml_str.contains("api_token = "));
        assert!(toml_str.contains('"'));
    }

    #[test]
    fn test_config_default_values() {
        // Verify Config struct can be created with default values
        let config = Config {
            api_token: String::new(),
        };
        assert_eq!(config.api_token, "");
    }
}
