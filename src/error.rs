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

pub type Result<T> = std::result::Result<T, TodoError>;
