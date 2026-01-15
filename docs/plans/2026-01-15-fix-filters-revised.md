# Fix Filters and Output Format - REVISED IMPLEMENTATION PLAN

> **Based on Official Todoist API v1 Documentation**
>
> **Critical Discovery from Official Docs:**
> - The `/api/v1/tasks` endpoint NO LONGER supports `filter` parameter in API v1
> - Must use `/api/v1/tasks/filter` endpoint with `query` parameter
> - Field name is `checked` (not `is_completed`) in API response

**Goal:** Fix filter and output format bugs using correct API v1 endpoints

---

## Root Cause Analysis

### Problem 1: Wrong Endpoint for Filtering
**Current Code** (`src/api.rs:46-87`):
```rust
pub async fn get_tasks(&self, filter: Option<String>) -> Result<Vec<TaskOutput>, TodoError> {
    let mut request = self.http.get(format!("{}/tasks", self.base_url));
    if let Some(filter_str) = filter {
        request = request.query(&[("filter", filter_str)]); // ❌ WRONG!
    }
}
```

**Issue**: Uses `/api/v1/tasks?filter=...` but this endpoint doesn't support `filter` parameter in API v1.

**Solution**: Use `/api/v1/tasks/filter?query=...` instead.

### Problem 2: Parameter Name Mismatch
- Current code uses `filter` as query parameter name
- API v1 requires `query` as parameter name for `/api/v1/tasks/filter` endpoint

### Problem 3: Field Mapping Actually Works
- API returns `checked: false/true` field
- Current model has `#[serde(alias = "checked")]` which should handle this
- BUT: we need to verify the serde alias is working correctly

---

## Implementation Tasks

### Task 1: Update `get_tasks` to Use Correct Endpoint

**File**: `src/api.rs`

**Step 1: Modify the endpoint URL and parameter name**

Replace the current implementation (lines 46-87) with:

```rust
pub async fn get_tasks(
    &self,
    filter: Option<String>,
) -> Result<Vec<TaskOutput>, crate::error::TodoError> {
    // Use the /filter endpoint when filter is provided, /tasks for no filter
    let endpoint = if filter.is_some() {
        format!("{}/tasks/filter", self.base_url)
    } else {
        format!("{}/tasks", self.base_url)
    };

    let mut request = self
        .http
        .get(&endpoint)
        .header("Authorization", self.get_auth_header());

    // Use 'query' parameter name for filter endpoint
    if let Some(filter_str) = filter {
        request = request.query(&[("query", &filter_str)]);
    }

    let response = request.send().await?;

    let status = response.status();
    let response_text = response.text().await?;

    if !status.is_success() {
        return Err(TodoError::Http(status.as_u16(), response_text));
    }

    let tasks_response: TasksResponse = serde_json::from_str(&response_text)
        .map_err(|e| TodoError::Api(format!("Failed to parse tasks response: {}\nResponse: {}", e, response_text)))?;
    Ok(self.enrich_tasks(tasks_response.results).await)
}
```

**Key Changes**:
1. Uses `/tasks/filter` endpoint when filter is provided
2. Uses `query` parameter name (not `filter`)
3. Uses `/tasks` endpoint when no filter (for getting all active tasks)

**Step 2: Build and test**

```bash
cargo build
```

**Step 3: Test with priority filter**

```bash
todorust tasks --filter 'priority:4' --format json 2>/dev/null | jq '.[0].priority'
```

Expected: Should return `4` (or no results if no priority 4 tasks exist).

**Step 4: Test with completed filter**

```bash
todorust tasks --filter 'completed today' --format checklist
```

Expected: Completed tasks should show `- [x]`.

**Step 5: Commit changes**

```bash
git add src/api.rs
git commit -m "fix: use correct /api/v1/tasks/filter endpoint with query parameter"
```

---

### Task 2: Verify and Fix Field Mapping

**File**: `src/models.rs`

**Step 1: Check current Task struct definition**

Read lines 12-28 to verify the serde alias is correct:

```rust
#[serde(alias = "checked")]
pub is_completed: bool,
```

**Step 2: If needed, update the field mapping**

Based on the official API response you provided, the field is `checked: false/true`.
Ensure the Task struct has:

```rust
#[serde(default, alias = "checked")]
pub is_completed: bool,
```

The `default` attribute ensures it defaults to `false` if the field is missing.

**Step 3: Test deserialization**

Add a test to verify:

```rust
#[test]
fn test_task_deserialization_with_checked_field() {
    let json = r#"{
        "id": "123",
        "content": "Test task",
        "checked": true,
        "added_at": "2026-01-15T10:00:00Z",
        "child_order": 1,
        "priority": 4,
        "labels": []
    }"#;

    let task: crate::models::Task = serde_json::from_str(json).unwrap();
    assert_eq!(task.is_completed, true);
}
```

**Step 4: Run tests**

```bash
cargo test test_task_deserialization_with_checked_field --lib
```

**Step 5: Commit if changes were needed**

```bash
git add src/models.rs
git commit -m "fix: ensure task completion field mapping handles 'checked' from API"
```

---

### Task 3: Remove Debug Logging

**File**: `src/api.rs`

**Step 1: Remove debug logging**

Remove the debug eprintln! statements added previously (lines 55-78):
- `eprintln!("DEBUG: Filter string: {:?}", filter_str);`
- `eprintln!("DEBUG: Filter contains special chars...");`
- `eprintln!("DEBUG: Raw API response for filter '{:?}':", filter_debug);`
- `eprintln!("{}", response_text);`

**Step 2: Clean up code**

Remove any unused variables created for debugging (like `filter_debug`).

**Step 3: Test to ensure functionality still works**

```bash
cargo build
cargo test --lib
```

**Step 4: Commit**

```bash
git add src/api.rs
git commit -m "chore: remove debug logging after verification"
```

---

### Task 4: Update Documentation

**File**: `README.md` (if it exists)

**Step 1: Update examples to show correct filter usage**

Ensure all examples use the filter syntax correctly:

```bash
# Get priority 4 tasks
todorust tasks --filter 'priority:4'

# Get completed tasks from today
todorust tasks --filter 'completed today'

# Get tasks due today
todorust tasks --filter 'due today'
```

**Step 2: Document supported filter types**

Based on Todoist's filter syntax, add examples of common filters:
- `priority:4` - Priority 4 (urgent)
- `completed today` - Completed today
- `due today` - Due today
- `project:Work` - In Work project
- `no due date` - Tasks without due date

**Step 3: Commit**

```bash
git add README.md
git commit -m "docs: update filter examples with correct syntax"
```

---

### Task 5: Add Integration Tests

**File**: `src/api.rs` (tests module)

**Step 1: Add test for priority filter**

```rust
#[tokio::test]
#[ignore]
async fn test_priority_filter_works() {
    let client = TodoistClient::new(get_test_token());

    // Create a priority 4 task
    let task = client
        .create_task("Test priority filter", None, None, Some(4), None)
        .await
        .unwrap();

    // Wait for API to process
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // Filter by priority 4
    let tasks = client.get_tasks(Some("priority:4".to_string())).await.unwrap();

    // Verify all returned tasks have priority 4
    for t in &tasks {
        assert_eq!(t.priority, 4, "Priority filter should only return priority 4 tasks");
    }

    // Cleanup
    let _ = client.delete_task(&task.id).await;
}
```

**Step 2: Add test for completed status**

```rust
#[tokio::test]
#[ignore]
async fn test_completed_filter_shows_correct_checkbox() {
    use crate::{Formattable, OutputFormat};
    let client = TodoistClient::new(get_test_token());

    // Create and complete a task
    let task = client
        .create_task("Test completed status", None, None, None, None)
        .await
        .unwrap();

    client.complete_task(&task.id).await.unwrap();

    // Wait for API to process
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Get completed tasks
    let tasks = client.get_tasks(Some("completed today".to_string())).await.unwrap();

    // Verify checklist format shows [x]
    let output = tasks.format(&OutputFormat::Checklist);

    // Cleanup
    let _ = client.delete_task(&task.id).await;

    // Assertions
    if !tasks.is_empty() {
        assert!(output.contains("[x]"), "Completed tasks should show [x] in checklist format");
    }
}
```

**Step 3: Run integration tests**

```bash
cargo test -- --ignored
```

**Step 4: Commit**

```bash
git add src/api.rs
git commit -m "test: add integration tests for filter and completion status"
```

---

## Testing Strategy

### Manual Testing Checklist

**Test 1: Priority filtering**
```bash
# Create test task
todorust create --content "Urgent task" --priority 4

# Filter by priority
todorust tasks --filter 'priority:4' --format json | jq '.[].priority'
```

Expected: Should only show `4`.

**Test 2: Completed tasks in checklist format**
```bash
# Create and complete a task
todorust create --content "Test task"
todorust complete --task_id <task_id>

# Query completed tasks
todorust tasks --filter 'completed today' --format checklist
```

Expected: Should show `- [x] Test task (Inbox)`.

**Test 3: Complex filters**
```bash
todorust tasks --filter 'priority:4 & project:Work' --format structured
```

Expected: Should show only priority 4 tasks in Work project.

**Test 4: All tasks (no filter)**
```bash
todorust tasks --format checklist | head -20
```

Expected: Should show all active tasks.

---

## Success Criteria

✅ `todorust tasks --filter 'priority:4'` returns only tasks with priority 4
✅ `todorust tasks --filter 'completed today' --format checklist` shows `[x]` for completed tasks
✅ `todorust tasks` (no filter) returns all active tasks
✅ All existing tests pass
✅ New integration tests pass
✅ No debug logging in production code

---

## API Reference Summary

**Endpoint for filtering**: `GET /api/v1/tasks/filter`

**Query Parameters**:
- `query` (required): Filter string (1-1024 characters)
- `lang` (optional): Language tag
- `cursor` (optional): For pagination
- `limit` (optional): Number of results (default 50, max 200)

**Response Fields**:
- `results`: Array of task objects
- `checked`: Boolean field for completion status
- `completed_at`: String with completion timestamp

**Key Changes from Old Implementation**:
1. Endpoint: `/api/v1/tasks` → `/api/v1/tasks/filter`
2. Parameter: `filter` → `query`
3. Field: API returns `checked` (handled by serde alias)

---

## Notes

- Todoist API v1 migration changed filter endpoint from `/tasks?filter=` to `/tasks/filter?query=`
- The `checked` field is the correct field name in API v1 response
- Current code's `#[serde(alias = "checked")]` should handle this automatically
- Pagination is supported via `cursor` parameter (not implemented in current code)
