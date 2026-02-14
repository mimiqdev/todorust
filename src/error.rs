/*!
 * # Error Types
 *
 * Central error handling for the application.
 * All errors are defined in this module as a `TodoError` enum.
 */

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TodoError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("API request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Todoist API error: {0}")]
    Api(String),

    #[error("Todoist API returned HTTP {0}")]
    Http(u16),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Configuration not found. Please run `todorust init --api-token YOUR_TOKEN` to set up your Todoist API token.")]
    ConfigNotFound,

    #[error("Serialization failed: {0}")]
    Serialize(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_todo_error_config_message() {
        let error = TodoError::Config("test error message".to_string());
        assert_eq!(
            format!("{}", error),
            "Configuration error: test error message"
        );
    }

    #[test]
    fn test_todo_error_api_message() {
        let error = TodoError::Api("api error".to_string());
        assert_eq!(format!("{}", error), "Todoist API error: api error");
    }

    #[test]
    fn test_todo_error_http_message() {
        let error = TodoError::Http(404);
        assert_eq!(format!("{}", error), "Todoist API returned HTTP 404");
    }

    #[test]
    fn test_todo_error_invalid_input_message() {
        let error = TodoError::InvalidInput("invalid input".to_string());
        assert_eq!(format!("{}", error), "Invalid input: invalid input");
    }

    #[test]
    fn test_todo_error_config_not_found_message() {
        let error = TodoError::ConfigNotFound;
        assert_eq!(
            format!("{}", error),
            "Configuration not found. Please run `todorust init --api-token YOUR_TOKEN` to set up your Todoist API token."
        );
    }

    #[test]
    fn test_todo_error_serialize_message() {
        let error = TodoError::Serialize("serialization failed".to_string());
        assert_eq!(
            format!("{}", error),
            "Serialization failed: serialization failed"
        );
    }

    #[test]
    fn test_todo_error_from_serde_json() {
        let json_error = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
        let todo_error: TodoError = json_error.into();
        assert!(format!("{}", todo_error).contains("JSON error"));
    }

    #[test]
    fn test_todo_error_from_toml_de() {
        // Create a proper toml::de::Error by attempting to parse invalid toml
        let invalid_toml = "invalid = [";
        let parse_result: std::result::Result<toml::Value, toml::de::Error> = invalid_toml.parse();
        assert!(parse_result.is_err());

        let toml_err = parse_result.unwrap_err();
        let todo_error: TodoError = toml_err.into();
        assert!(format!("{}", todo_error).contains("Parse error"));
    }

    #[test]
    fn test_result_type_alias() {
        fn returns_result() -> Result<i32> {
            Ok(42)
        }

        let result = returns_result();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_result_type_alias_error() {
        fn returns_error() -> Result<i32> {
            Err(TodoError::Config("error".to_string()))
        }

        let result = returns_error();
        assert!(result.is_err());
    }

    #[test]
    fn test_todo_error_from_io() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "not found");
        let todo_error: TodoError = io_error.into();
        assert!(matches!(todo_error, TodoError::Io(_)));
        assert_eq!(format!("{}", todo_error), "IO error: not found");
    }

    #[test]
    fn test_todo_error_debug() {
        let error = TodoError::Config("debug test".to_string());
        let debug_format = format!("{:?}", error);
        assert!(debug_format.contains("Config"));
        assert!(debug_format.contains("debug test"));
    }
}
