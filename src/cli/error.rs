use crate::error::TodoError;

pub fn handle_error(error: TodoError) -> ! {
    match &error {
        TodoError::ConfigNotFound => {
            eprintln!("Error: Configuration not found.");
            eprintln!("To get started, obtain your API token from https://todoist.com/app/settings/integrations");
            eprintln!("Then run: todorust init --api-token YOUR_TOKEN");
        }
        TodoError::Http(status) => {
            match status {
                401 => eprintln!("Error: Unauthorized (401). Your API token might be invalid or expired."),
                403 => eprintln!("Error: Forbidden (403). You don't have permission to perform this action."),
                404 => eprintln!("Error: Not Found (404). The requested resource was not found."),
                429 => eprintln!("Error: Too Many Requests (429). Todoist API rate limit exceeded. Please wait a moment."),
                _ => eprintln!("Error: Todoist API returned HTTP {}.", status),
            }
        }
        TodoError::Api(msg) => {
            eprintln!("Todoist API Error: {}", msg);
        }
        TodoError::Request(e) => {
            eprintln!("Network Request Error: {}", e);
            if e.is_connect() {
                eprintln!("Hint: Check your internet connection.");
            }
        }
        TodoError::Config(msg) => {
            eprintln!("Configuration Error: {}", msg);
        }
        TodoError::InvalidInput(msg) => {
            eprintln!("Invalid Input: {}", msg);
        }
        TodoError::Serialize(msg) => {
            eprintln!("Data Processing Error: {}", msg);
        }
        TodoError::Io(e) => {
            eprintln!("System IO Error: {}", e);
        }
    }
    std::process::exit(1);
}
