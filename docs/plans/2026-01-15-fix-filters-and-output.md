# Fix Filters and Output Format Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Fix filter and output format bugs where completed tasks display incorrectly, priority filters don't work, and checklist format shows wrong completion status.

**Architecture:** The current implementation passes filters directly to Todoist API, but has three bugs:
1. API field name mismatch (`checked` vs `is_completed`) causing completed tasks to appear incomplete
2. Checklist format reads `is_completed` correctly but data from API is wrong
3. Priority filter may not be URL-encoded properly

**Tech Stack:** Rust, serde, reqwest, Todoist REST API v1

---

## Problem Analysis

### Bug 1: Completed Tasks Show as Incomplete in Checklist Format

**Symptom:** `todorust tasks --filter 'completed today' --format checklist` shows `- [ ]` instead of `- [x]`

**Root Cause:** Todoist API v1 `/tasks` endpoint returns `is_completed` field for task completion status. However, the API might use different field names depending on the endpoint or query parameters. The current model has `#[serde(alias = "checked")]` but the actual field from the API might be different.

**Evidence from code:**
- `src/models.rs:19-20` - Task struct defines `is_completed` with alias `checked`
- `src/formatter.rs:32` - Checklist format correctly reads `task.is_completed`
- `src/api.rs:55-56` - Filter is passed as query param to API

### Bug 2: Priority Filter Returns Wrong Priority Values

**Symptom:** `todorust tasks --filter 'priority:4' --format json` returns tasks with `priority: 1`

**Root Cause:** The filter string might need URL encoding when passed as query parameter. The space in "priority:4" is fine, but there could be encoding issues with special characters.

**Evidence from code:**
- `src/api.rs:55-56` - Uses `request.query(&[("filter", filter_str)])`
- reqwest should handle encoding, but we need to verify

### Bug 3: API Endpoint Limitation

**Critical Discovery:** Todoist's `/tasks` endpoint (REST API v1) only returns active/incomplete tasks by default. To get completed tasks, you need to use different approaches:
1. Use the `completed` endpoint: `/completed/get_all` (Sync API v9)
2. Or use the filter with specific parameters that include completed tasks

The current code uses REST API v1 `/tasks` endpoint which doesn't support retrieving completed tasks properly.

---

## Solution Strategy

### Phase 1: Verify API Field Names

First, we need to understand what the Todoist API actually returns. We'll add debug logging to see the raw response.

### Phase 2: Fix the Serde Field Mapping

Based on API documentation and actual response, update the field mapping to correctly deserialize `is_completed`.

### Phase 3: Add URL Encoding Verification

Ensure filter strings are properly URL-encoded when sent to API.

### Phase 4: (Optional) Add Support for Completed Tasks Endpoint

If the current endpoint cannot return completed tasks, add support for the completed tasks endpoint.

---

## Implementation Tasks

### Task 1: Add Debug Logging to Verify API Response

**Files:**
- Modify: `src/api.rs:59-68` (in `get_tasks` method)

**Step 1: Add debug logging before parsing**

Open `src/api.rs` and locate the `get_tasks` method around line 46-70.

After line 61 where `response_text` is obtained, add debug logging:

```rust
let response_text = response.text().await?;

// DEBUG: Log raw API response to verify field names
eprintln!("DEBUG: Raw API response for filter '{:?}':", filter);
eprintln!("{}", response_text);
```

**Step 2: Rebuild and test with completed filter**

Run: `cargo build`

Run: `todorust tasks --filter 'completed today' --format json 2>/dev/null | head -50`

Expected: You'll see the raw JSON response from Todoist API. Look for what field name indicates completion status.

**Step 3: Test with priority filter**

Run: `todorust tasks --filter 'priority:4' --format json 2>/dev/null | head -50`

Expected: See raw JSON to verify if priority filtering works at API level.

**Step 4: Commit debug logging**

```bash
git add src/api.rs
git commit -m "debug: add logging to verify Todoist API response structure"
```

---

### Task 2: Inspect API Response and Update Field Mapping

**Files:**
- Modify: `src/models.rs:13-28` (Task struct)
- Test: `src/api.rs:309-347` (existing deserialization tests)

**Step 1: Check Todoist API documentation**

Based on Todoist API v1 documentation for Tasks endpoint:
- Field is `is_completed` (boolean) - indicates if task is completed
- Field `checked` is NOT used in REST API v1, it's from Sync API v9

**Step 2: Verify actual response from debug logging**

From Task 1 debug output, check if the API returns:
- `is_completed: true/false` (REST API v1 format)
- `checked: true/false` (Sync API v9 format - should NOT appear)
- `completed: true/false` (possible alternative)

**Step 3: Update serde field mapping based on findings**

If the API returns a different field name, update the Task struct in `src/models.rs`:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub content: String,
    pub project_id: Option<String>,
    #[serde(default)]
    pub due: Option<Due>,
    #[serde(alias = "checked", alias = "completed")]
    pub is_completed: bool,
    // ... rest of fields
}
```

**Step 4: Add test case for completed task deserialization**

Add to `src/api.rs` in the `tests` module after line 347:

```rust
#[test]
fn test_task_deserialization_completed() {
    let json = r#"{
        "id": "123",
        "content": "Completed task",
        "project_id": "456",
        "is_completed": true,
        "created_at": "2026-01-15T10:00:00Z",
        "order": 1,
        "priority": 4,
        "labels": []
    }"#;

    let task: crate::models::Task = serde_json::from_str(json).unwrap();
    assert_eq!(task.is_completed, true);
    assert_eq!(task.content, "Completed task");
}

#[test]
fn test_task_deserialization_with_checked_alias() {
    let json = r#"{
        "id": "123",
        "content": "Task with checked field",
        "checked": false,
        "created_at": "2026-01-15T10:00:00Z",
        "order": 1,
        "priority": 2,
        "labels": []
    }"#;

    let task: crate::models::Task = serde_json::from_str(json).unwrap();
    assert_eq!(task.is_completed, false);
}
```

**Step 5: Run tests**

Run: `cargo test api::tests::test_task_deserialization --lib`

Expected: Both new tests pass, confirming serde correctly handles field names.

**Step 6: Commit changes**

```bash
git add src/models.rs src/api.rs
git commit -m "fix: update task completion field mapping for API compatibility"
```

---

### Task 3: Verify and Fix Filter URL Encoding

**Files:**
- Modify: `src/api.rs:55-56` (filter query parameter)
- Test: `src/api.rs` (add integration test)

**Step 1: Verify reqwest's default encoding**

Check if reqwest's `query()` method properly encodes special characters.

Add test in `src/api.rs` tests module after line 543:

```rust
#[test]
fn test_filter_url_encoding() {
    use reqwest::Client;
    let client = Client::new();

    // Build a request to see how reqwest encodes the query
    let request = client
        .get("https://api.todoist.com/api/v1/tasks")
        .query(&[("filter", "priority:4 & completed today")]);

    // Get the request URL to verify encoding
    let url = request.url_ref().unwrap();
    assert!(url.as_str().contains("filter="));
    eprintln!("Encoded URL: {}", url);
}
```

**Step 2: Run the encoding test**

Run: `cargo test api::tests::test_filter_url_encoding --lib -- --nocapture`

Expected: See how reqwest encodes the filter string. Look for proper URL encoding of spaces and special characters.

**Step 3: If encoding is wrong, implement manual encoding**

If the test shows incorrect encoding, update `src/api.rs:55-56`:

Replace:
```rust
if let Some(filter_str) = filter {
    request = request.query(&[("filter", filter_str)]);
}
```

With:
```rust
if let Some(filter_str) = filter {
    // Manually URL-encode the filter to ensure proper encoding
    use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
    let encoded = utf8_percent_encode(&filter_str, NON_ALPHANUMERIC).to_string();
    request = request.query(&[("filter", encoded)]);
}
```

And add to `Cargo.toml` dependencies:
```toml
percent-encoding = "2.3"
```

**Step 4: Test with real API**

Run: `cargo build`

Run: `todorust tasks --filter 'priority:4' --format json | jq '.[0].priority'`

Expected: Should return `4` (or no results if no priority 4 tasks exist).

**Step 5: Commit**

```bash
git add src/api.rs Cargo.toml
git commit -m "fix: ensure proper URL encoding for filter parameters"
```

---

### Task 4: Remove Debug Logging

**Files:**
- Modify: `src/api.rs:59-68`

**Step 1: Remove the debug logging added in Task 1**

Remove the eprintln! statements added in Task 1, Step 2.

**Step 2: Commit**

```bash
git add src/api.rs
git commit -m "chore: remove debug logging after verification"
```

---

### Task 5: Add Integration Tests for Filter Behavior

**Files:**
- Modify: `src/api.rs` (add integration tests)

**Step 1: Add test for completed status in output**

Add to `src/api.rs` tests module:

```rust
#[tokio::test]
#[ignore]
async fn test_checklist_shows_completed_status() {
    use crate::{Formattable, OutputFormat};
    let client = TodoistClient::new(get_test_token());

    // Create a test task
    let task = client
        .create_task("Test completed status", None, None, None, None)
        .await
        .unwrap();

    // Complete it
    client.complete_task(&task.id).await.unwrap();

    // Wait a moment for API to process
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // Try to get completed tasks
    let tasks = client.get_tasks(Some("completed today".to_string())).await.unwrap();

    // Verify the checkbox shows [x] for completed tasks
    let output = tasks.format(&OutputFormat::Checklist);

    // Cleanup
    let _ = client.delete_task(&task.id).await;

    // Assertions
    if !tasks.is_empty() {
        assert!(output.contains("[x]"), "Completed task should show [x] in checklist format");
    }
}
```

**Step 2: Add test for priority filtering**

Add to `src/api.rs` tests module:

```rust
#[tokio::test]
#[ignore]
async fn test_priority_filter() {
    let client = TodoistClient::new(get_test_token());

    // Create tasks with different priorities
    let task_p4 = client
        .create_task("Priority 4 task", None, None, Some(4), None)
        .await
        .unwrap();

    let task_p1 = client
        .create_task("Priority 1 task", None, None, Some(1), None)
        .await
        .unwrap();

    // Wait for API to process
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // Filter by priority 4
    let tasks = client.get_tasks(Some("priority:4".to_string())).await.unwrap();

    // Verify all returned tasks have priority 4
    for task in &tasks {
        assert_eq!(task.priority, 4, "Priority filter should only return priority 4 tasks");
    }

    // Cleanup
    let _ = client.delete_task(&task_p4.id).await;
    let _ = client.delete_task(&task_p1.id).await;
}
```

**Step 3: Run integration tests**

Run: `cargo test --api -- --ignored`

Expected: Tests pass, confirming filter behavior works correctly.

**Step 4: Commit**

```bash
git add src/api.rs
git commit -m "test: add integration tests for filter and format behavior"
```

---

## Testing Strategy

### Manual Testing Checklist

After implementing all tasks, test these scenarios:

**Test 1: Completed tasks in checklist format**
```bash
# First complete a task using the CLI
todorust create --content "Test completed task"
# Note the task_id from output
todorust complete --task_id <task_id>

# Now query for completed tasks
todorust tasks --filter 'completed today' --format checklist
```

Expected: Should show `- [x] Test completed task (Inbox)`

**Test 2: Priority filtering**
```bash
# Create high priority task
todorust create --content "Urgent task" --priority 4

# Query for priority 4 tasks
todorust tasks --filter 'priority:4' --format json | jq '.[].priority'
```

Expected: Should only show `4` values (or be empty if no priority 4 tasks).

**Test 3: Complex filters**
```bash
todorust tasks --filter 'priority:4 & project:Inbox' --format structured
```

Expected: Should show only priority 4 tasks in Inbox project.

**Test 4: Checklist format shows correct checkboxes**
```bash
todorust tasks --filter 'no due date' --format checklist | head -20
```

Expected: All tasks should show `- [ ]` for incomplete tasks.

---

## Rollback Plan

If any task breaks functionality:

1. **Revert the commit**: `git revert HEAD`
2. **Identify the issue**: Add more debug logging
3. **Fix and retry**: Create new commit with fix

---

## Notes

- Todoist API v1 `/tasks` endpoint documentation: https://developer.todoist.com/rest/v1/#get-tasks
- Filter syntax reference: https://developer.todoist.com/rest/v1/#query-params
- The `/tasks` endpoint may not return all completed tasks - consider using `/completed/get_all` from Sync API v9 if needed
- Priority in Todoist: 4=urgent, 3=high, 2=normal, 1=low (counterintuitive!)

---

## Success Criteria

✅ `todorust tasks --filter 'completed today' --format checklist` shows `[x]` for completed tasks
✅ `todorust tasks --filter 'priority:4' --format json` returns only tasks with priority 4
✅ All existing tests still pass
✅ New integration tests pass
✅ No debug logging in production code
