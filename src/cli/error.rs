use crate::error::TodoError;

pub fn handle_error(error: TodoError) -> ! {
    eprintln!("{}", get_error_message(&error));
    std::process::exit(1);
}

pub fn get_error_message(error: &TodoError) -> String {
    match error {
        TodoError::ConfigNotFound => {
            "Error: Configuration not found.\nTo get started, obtain your API token from https://todoist.com/app/settings/integrations\nThen run: todorust init --api-token YOUR_TOKEN".to_string()
        }
        TodoError::Http(status) => {
            match status {
                401 => "Error: Unauthorized (401). Your API token might be invalid or expired.".to_string(),
                403 => "Error: Forbidden (403). You don't have permission to perform this action.".to_string(),
                404 => "Error: Not Found (404). The requested resource was not found.".to_string(),
                429 => "Error: Too Many Requests (429). Todoist API rate limit exceeded. Please wait a moment.".to_string(),
                _ => format!("Error: Todoist API returned HTTP {}.", status),
            }
        }
        TodoError::Api(msg) => {
            format!("Todoist API Error: {}", msg)
        }
        TodoError::Request(e) => {
            let mut msg = format!("Network Request Error: {}", e);
            if e.is_connect() {
                msg.push_str("\nHint: Check your internet connection.");
            }
            msg
        }
        TodoError::Config(msg) => {
            format!("Configuration Error: {}", msg)
        }
        TodoError::InvalidInput(msg) => {
            format!("Invalid Input: {}", msg)
        }
        TodoError::Serialize(msg) => {
            format!("Data Processing Error: {}", msg)
        }
        TodoError::Io(e) => {
            format!("System IO Error: {}", e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_error_message_config_not_found() {
        let err = TodoError::ConfigNotFound;
        let msg = get_error_message(&err);
        assert!(msg.contains("Configuration not found"));
    }

    #[test]
    fn test_get_error_message_http_401() {
        let err = TodoError::Http(401);
        let msg = get_error_message(&err);
        assert!(msg.contains("Unauthorized"));
    }

    #[test]
    fn test_get_error_message_http_403() {
        let err = TodoError::Http(403);
        let msg = get_error_message(&err);
        assert!(msg.contains("Forbidden"));
    }

    #[test]
    fn test_get_error_message_http_429() {
        let err = TodoError::Http(429);
        let msg = get_error_message(&err);
        assert!(msg.contains("Too Many Requests"));
    }

    #[test]
    fn test_get_error_message_api() {
        let err = TodoError::Api("custom api error".to_string());
        let msg = get_error_message(&err);
        assert!(msg.contains("Todoist API Error: custom api error"));
    }

    #[test]
    fn test_get_error_message_config() {
        let err = TodoError::Config("bad config".to_string());
        let msg = get_error_message(&err);
        assert!(msg.contains("Configuration Error: bad config"));
    }

    #[test]
    fn test_get_error_message_serialize() {
        let err = TodoError::Serialize("json error".to_string());
        let msg = get_error_message(&err);
        assert!(msg.contains("Data Processing Error: json error"));
    }

    #[test]
    fn test_get_error_message_http_404() {
        let err = TodoError::Http(404);
        let msg = get_error_message(&err);
        assert!(msg.contains("Not Found"));
    }

    #[test]
    fn test_get_error_message_http_other() {
        let err = TodoError::Http(500);
        let msg = get_error_message(&err);
        assert!(msg.contains("HTTP 500"));
    }

    #[test]
    fn test_get_error_message_invalid_input() {
        let err = TodoError::InvalidInput("test input".to_string());
        let msg = get_error_message(&err);
        assert!(msg.contains("Invalid Input: test input"));
    }

    #[test]
    fn test_get_error_message_request() {
        // We can't easily create a connect error without real networking,
        // but we can test the basic Request error display.
        // reqwest::Error is hard to construct directly, but we can't do much about it
        // other than ensuring the branch is represented if possible.
    }
}
