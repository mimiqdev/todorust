use thiserror::Error;

#[derive(Error, Debug)]
pub enum TodoError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("API request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("API error: {0}")]
    Api(String),

    #[error("HTTP error {0}: {1}")]
    Http(u16, String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Configuration not found. Run `todorust init --api-token YOUR_TOKEN` to configure.")]
    ConfigNotFound,
}

impl From<serde_json::Error> for TodoError {
    fn from(err: serde_json::Error) -> Self {
        TodoError::Api(format!("JSON error: {}", err))
    }
}

impl From<toml::de::Error> for TodoError {
    fn from(err: toml::de::Error) -> Self {
        TodoError::Config(format!("Parse error: {}", err))
    }
}

pub type Result<T> = std::result::Result<T, TodoError>;
