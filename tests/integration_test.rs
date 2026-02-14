use std::env;

#[tokio::test]
#[ignore]
async fn test_end_to_end_workflow() {
    // Get token from environment variable or config file
    let token = env::var("TODORUST_API_TOKEN")
        .or_else(|_| env::var("TODOIST_API_TOKEN"))
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

    // Test Task Lifecycle
    println!("Testing task lifecycle (create -> complete -> reopen -> delete)...");

    let task_id = client
        .add_task(
            "Integration Test Task",
            Some("Created by integration test"),
            None,
            None,
            None,
            Some(4),
            None,
        )
        .await
        .expect("Failed to create task");
    println!("Created task with ID: {}", task_id);

    client
        .complete_task(&task_id)
        .await
        .expect("Failed to complete task");
    println!("Completed task {}", task_id);

    client
        .reopen_task(&task_id)
        .await
        .expect("Failed to reopen task");
    println!("Reopened task {}", task_id);

    client
        .delete_task(&task_id)
        .await
        .expect("Failed to delete task");
    println!("Deleted task {}", task_id);
}
