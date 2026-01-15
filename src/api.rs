use reqwest::Client as HttpClient;

pub struct TodoistClient {
    token: String,
    base_url: String,
    http: HttpClient,
}

impl TodoistClient {
    pub fn new(token: String) -> Self {
        Self {
            token,
            base_url: "https://api.todoist.com/api/v1".to_string(),
            http: HttpClient::new(),
        }
    }

    fn get_auth_header(&self) -> String {
        format!("Bearer {}", self.token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = TodoistClient::new("test_token".to_string());
        assert_eq!(client.token, "test_token");
    }

    #[test]
    fn test_base_url() {
        let client = TodoistClient::new("test_token".to_string());
        assert_eq!(client.base_url, "https://api.todoist.com/api/v1");
    }

    #[test]
    fn test_get_auth_header() {
        let client = TodoistClient::new("test_token".to_string());
        let auth = client.get_auth_header();
        assert_eq!(auth, "Bearer test_token");
    }
}
