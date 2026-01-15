use thiserror::Error;

#[derive(Error, Debug)]
pub enum TodoError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("API request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("API error: {0}")]
    Api(String),

    #[error("HTTP error: {0}")]
    Http(u16, String),
}

impl From<serde_json::Error> for TodoError {
    fn from(err: serde_json::Error) -> Self {
        TodoError::Api(format!("JSON error: {}", err))
    }
}

pub type Result<T> = std::result::Result<T, TodoError>;
