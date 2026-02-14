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
    fn test_get_error_message_invalid_input() {
        let err = TodoError::InvalidInput("test".to_string());
        let msg = get_error_message(&err);
        assert!(msg.contains("Invalid Input: test"));
    }
}
