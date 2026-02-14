use std::env;

#[tokio::test]
#[ignore]
async fn test_end_to_end_workflow() {
    // Get token from environment variable or config file
    let token = env::var("TODOIST_API_TOKEN")
        .ok()
        .or_else(|| {
            // Try loading from config file
            todorust::config::load_config()
                .ok()
                .map(|config| config.api_token)
        })
        .expect("TODOIST_API_TOKEN env var or config file required");

    println!("Integration test starting with real API...");

    let client = todorust::sync::TodoistSyncClient::new(token);

    // Test projects retrieval
    let projects = client
        .get_projects()
        .await
        .expect("Failed to fetch projects");
    println!("Successfully fetched {} projects", projects.len());

    // Test tasks retrieval
    let tasks = client.get_tasks().await.expect("Failed to fetch tasks");
    println!("Successfully fetched {} tasks", tasks.len());

    // Verify we got some data if the account is not empty
    // (If it's empty, we just verify the call succeeded)
}
