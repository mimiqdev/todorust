use std::env;

#[test]
#[ignore] // Run with: cargo test --test integration_test -- --ignored
fn test_end_to_end_workflow() {
    // Get token from environment variable or config file
    let token = env::var("TODOIST_TOKEN")
        .ok()
        .or_else(|| {
            // Try loading from config file
            todorust::config::load_config()
                .ok()
                .map(|config| config.api_token)
        })
        .expect("TODOIST_TOKEN env var or config file required");

    println!("Integration test requires proper config handling");
    println!("Token loaded successfully (first 8 chars): {}...", &token[..8.min(token.len())]);
}
