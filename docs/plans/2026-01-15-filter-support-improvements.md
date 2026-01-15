# Filter Support Improvements Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Improve filter syntax support by adding helpful error messages for unsupported syntax and implementing smart parsing for date ranges.

**Architecture:**
- Detect unsupported filter syntax and provide actionable error messages
- Parse `completed within "X days"` syntax and convert to `since/until` parameters
- Create a filter syntax compatibility layer for common patterns
- Add comprehensive filter syntax documentation

**Tech Stack:** Rust, chrono for date parsing, reqwest for HTTP requests, Todoist REST API v1

---

## Problem Analysis

### Current Issues

1. **HTTP 400 Errors for Unsupported Syntax**
   - `completed within "7 days of today"` → Returns 400 "INVALID_SEARCH_QUERY"
   - `no project` → Returns 400 "INVALID_SEARCH_QUERY"

2. **Root Causes**
   - `/tasks/filter` endpoint doesn't support `completed` queries (official limitation)
   - `/tasks/completed/by_completion_date` endpoint doesn't support `filter_query` with complex syntax
   - Some filter syntax in community docs don't exist in official API v1
   - No user-friendly error messages for unsupported syntax

3. **What Official Docs Say**
   - "Show completed tasks: Todoist filters show open or active tasks by default. Completed tasks aren't visible in filter views."
   - Supported: `no date`, `no labels`, `no time`, `no deadline`
   - **NOT found in docs**: `no project`, `completed within "..."`

### Solution Strategy

1. **Phase 1**: Add syntax validation and helpful error messages
2. **Phase 2**: Implement smart parsing for date ranges (`completed within "X days"`)
3. **Phase 3**: Add alias support (`no project` → `!project:Inbox`)
4. **Phase 4**: Update documentation with supported syntax

---

## Implementation Tasks

### Task 1: Add Filter Syntax Validation

**Goal**: Detect unsupported filter syntax and provide helpful error messages before making API calls

**Files:**
- Create: `src/filter_validator.rs`
- Modify: `src/api.rs:46-125` (get_tasks method)
- Modify: `src/lib.rs` (add module)
- Test: `src/filter_validator.rs` (tests module)

**Step 1: Create filter validator module**

Create `src/filter_validator.rs`:

```rust
use std::collections::HashSet;

/// Supported filter patterns
const SUPPORTED_PATTERNS: &[&str] = &[
    // Date filters
    "today", "tomorrow", "yesterday",
    "overdue", "no date", "no time",
    r"\d+ days", r"next \d+ days",
    r"due:", r"due before:", r"due after:",
    r"date:", r"date before:", r"date after:",
    r"created:", r"created before:", r"created after:",
    r"deadline:", r"no deadline",
    "recurring",

    // Priority filters
    r"priority:[1-4]", r"p[1-4]", r"no priority",
    r"priority >= [1-4]", r"priority <= [1-4]",

    // Project/Section filters
    r"#\w+", r"##\w+", r"/\w+",
    r"project:", r"!project:", r"!\*/",

    // Label filters
    r"@\w+", r"label:", r"no labels",

    // Assignment filters
    r"assigned to:", r"assigned by:", r"!assigned",
    "shared",

    // Search
    r"search:",

    // Completed (special handling)
    "completed",
];

/// Unsupported filter patterns with alternatives
const UNSUPPORTED_WITH_ALTERNATIVES: &[(&str, &str)] = &[
    ("no project", "Use '!project:Inbox' instead"),
    ("completed within", "Use date range or just 'completed today'"),
];

/// Validate filter syntax and return helpful error if unsupported
pub fn validate_filter_syntax(filter: &str) -> Result<(), String> {
    let filter_lower = filter.to_lowercase();

    // Check for known unsupported patterns
    for (pattern, alternative) in UNSUPPORTED_WITH_ALTERNATIVES {
        if filter_lower.contains(pattern) {
            return Err(format!(
                "Filter syntax '{}' is not supported by Todoist API v1.\n\
                 Alternative: {}\n\
                 See https://www.todoist.com/help/articles/introduction-to-filters-V98wIH",
                pattern, alternative
            ));
        }
    }

    // Check for completed with complex syntax
    if filter_lower.contains("completed") && !filter_lower.contains("completed today") {
        return Err(
            "Complex 'completed' filters are not fully supported.\n\
             Try: 'completed today' or use date ranges with 'due before/after'\n\
             Example: 'due before:today & due after:-7 days'".to_string()
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_supported_filters() {
        assert!(validate_filter_syntax("today").is_ok());
        assert!(validate_filter_syntax("priority:4").is_ok());
        assert!(validate_filter_syntax("#Work").is_ok());
        assert!(validate_filter_syntax("completed today").is_ok());
    }

    #[test]
    fn test_validate_unsupported_no_project() {
        let result = validate_filter_syntax("no project");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("!project:Inbox"));
    }

    #[test]
    fn test_validate_unsupported_completed_within() {
        let result = validate_filter_syntax("completed within \"7 days of today\"");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("completed today"));
    }
}
```

**Step 2: Add validator to lib.rs**

Modify `src/lib.rs`:

```rust
pub mod api;
pub mod config;
pub mod error;
pub mod filter_validator;
pub mod formatter;
pub mod models;

pub use formatter::{OutputFormat, Formattable};
pub use models::Project;
```

**Step 3: Integrate validator into get_tasks**

Modify `src/api.rs:46-47`:

```rust
pub async fn get_tasks(
    &self,
    filter: Option<String>,
) -> Result<Vec<TaskOutput>, crate::error::TodoError> {
    // Validate filter syntax before making API call
    if let Some(ref filter_str) = filter {
        crate::filter_validator::validate_filter_syntax(filter_str)
            .map_err(|e| TodoError::InvalidInput(e))?;
    }

    // Check if filter is asking for completed tasks
    let uses_completed_filter = filter.as_ref()
```

**Step 4: Run tests**

Run: `cargo test filter_validator --lib`

Expected: All tests pass

**Step 5: Test error messages manually**

```bash
# Test no project error
cargo run -- tasks --filter 'no project' --format json

# Test completed within error
cargo run -- tasks --filter 'completed within "7 days"' --format json
```

Expected: Helpful error messages with alternatives

**Step 6: Commit**

```bash
git add src/filter_validator.rs src/lib.rs src/api.rs
git commit -m "feat: add filter syntax validation with helpful error messages

- Detects unsupported syntax like 'no project' and 'completed within'
- Provides actionable alternatives in error messages
- Links to official Todoist filter documentation
- Validates before making API calls to save time

Resolves user confusion about HTTP 400 errors from unsupported filter syntax."
```

---

### Task 2: Implement Date Range Parsing for Completed Tasks

**Goal**: Parse `completed within "X days"` syntax and convert to `since/until` parameters

**Files:**
- Modify: `src/api.rs:71-91` (completed filter logic)

**Step 1: Add date range parsing function**

Add to `src/api.rs` before the `impl TodoistClient` block:

```rust
use chrono::{Utc, Duration, Datelike};

/// Parse "completed within \"X days of today\"" syntax
/// Returns (since_date, until_date) if pattern matches
fn parse_completed_date_range(filter: &str) -> Option<(String, String)> {
    let filter_lower = filter.to_lowercase();

    // Pattern: completed within "X days of today"
    // Example: completed within "7 days of today"
    if filter_lower.contains("completed within") {
        // Extract number between "within" and "days"
        let pattern_start = filter_lower.find("completed within")?;
        let after_within = &filter_lower[pattern_start + "completed within".len()..];

        // Find the number
        let num_str: String = after_within
            .chars()
            .skip_while(|c| c.is_whitespace() || *c == '"')
            .take_while(|c| c.is_numeric())
            .collect();

        let days: i64 = num_str.parse().ok()?;

        // Calculate date range
        let today = Utc::now().naive_utc().date();
        let since = today - Duration::days(days);
        let until = today;

        Some((
            format!("{}T00:00:00Z", since),
            format!("{}T23:59:59Z", until),
        ))
    } else if filter_lower.contains("completed today") {
        // Handle "completed today"
        let today = Utc::now().format("%Y-%m-%d").to_string();
        Some((
            format!("{}T00:00:00Z", today),
            format!("{}T23:59:59Z", today),
        ))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_completed_today() {
        let result = parse_completed_date_range("completed today");
        assert!(result.is_some());
        let (since, until) = result.unwrap();
        assert!(since.contains("T00:00:00Z"));
        assert!(until.contains("T23:59:59Z"));
    }

    #[test]
    fn test_parse_completed_within_days() {
        let result = parse_completed_date_range("completed within \"7 days of today\"");
        assert!(result.is_some());
        let (since, until) = result.unwrap();
        // since should be 7 days ago, until should be today
        assert!(since.contains("T00:00:00Z"));
        assert!(until.contains("T23:59:59Z"));
    }

    #[test]
    fn test_parse_no_date_range() {
        let result = parse_completed_date_range("priority:4");
        assert!(result.is_none());
    }
}
```

**Step 2: Update get_tasks to use date range parser**

Modify `src/api.rs:71-91`:

```rust
if uses_completed_filter {
    // For completed tasks, we need since/until dates
    // Try to parse date range from filter
    if let Some(ref filter_str) = filter {
        if let Some((since, until)) = parse_completed_date_range(filter_str) {
            request = request.query(&[("since", since)]);
            request = request.query(&[("until", until)]);
        } else {
            // Default to today if no date range found
            let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
            request = request.query(&[("since", format!("{}T00:00:00Z", today))]);
            request = request.query(&[("until", format!("{}T23:59:59Z", today))]);
        }

        // Remove date range parts from filter for filter_query
        let filter_query = filter_str
            .replace("completed within \"7 days of today\"", "")
            .replace("completed within", "")
            .replace("\"7 days of today\"", "")
            .replace("completed today", "")
            .replace("completed", "")
            .trim()
            .to_string();

        if !filter_query.is_empty() {
            request = request.query(&[("filter_query", &filter_query)]);
        }
    }
}
```

**Step 3: Run tests**

Run: `cargo test parse_completed_date_range --lib`

Expected: All tests pass

**Step 4: Test manually**

```bash
# Test completed within 7 days
cargo run -- tasks --filter 'completed within "7 days of today"' --format json

# Test completed today (should still work)
cargo run -- tasks --filter 'completed today' --format json
```

Expected: Both return results without HTTP 400 error

**Step 5: Commit**

```bash
git add src/api.rs
git commit -m "feat: parse date ranges for completed task filters

- Implements 'completed within \"X days of today\"' syntax
- Calculates since/until dates automatically
- Falls back to today if no date range detected
- Maintains backward compatibility with 'completed today'

Allows users to query completed tasks from specific date ranges."
```

---

### Task 3: Add Filter Alias Support

**Goal**: Support common filter aliases like `no project` → `!project:Inbox`

**Files:**
- Modify: `src/filter_validator.rs`
- Test: `src/filter_validator.rs` (add tests)

**Step 1: Add alias expansion function**

Add to `src/filter_validator.rs` after `validate_filter_syntax`:

```rust
/// Expand filter aliases to their full form
pub fn expand_filter_aliases(filter: &str) -> String {
    let mut expanded = filter.to_string();

    // Aliases map
    let aliases = [
        ("no project", "!project:Inbox"),
        ("!assigned", "!assigned to: anyone"),  // Alternative
    ];

    for (alias, replacement) in aliases {
        if expanded.to_lowercase().contains(alias) {
            expanded = expanded.replacen(alias, replacement, 1);
        }
    }

    expanded
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_no_project() {
        let result = expand_filter_aliases("no project & p1");
        assert_eq!(result, "!project:Inbox & p1");
    }

    #[test]
    fn test_expand_no_alias() {
        let result = expand_filter_aliases("priority:4");
        assert_eq!(result, "priority:4");
    }

    #[test]
    fn test_expand_with_other_filters() {
        let result = expand_filter_aliases("no project & today");
        assert!(result.contains("!project:Inbox"));
        assert!(result.contains("today"));
    }
}
```

**Step 2: Integrate alias expansion into get_tasks**

Modify `src/api.rs:47-49`:

```rust
pub async fn get_tasks(
    &self,
    filter: Option<String>,
) -> Result<Vec<TaskOutput>, crate::error::TodoError> {
    // Expand filter aliases before validation
    let filter = filter.map(|f| crate::filter_validator::expand_filter_aliases(&f));

    // Validate filter syntax before making API call
    if let Some(ref filter_str) = filter {
        crate::filter_validator::validate_filter_syntax(filter_str)
            .map_err(|e| TodoError::InvalidInput(e))?;
    }

    // Check if filter is asking for completed tasks
    let uses_completed_filter = filter.as_ref()
```

**Step 3: Update validator to allow aliases**

Modify `src/filter_validator.rs:26-35`:

```rust
/// Validate filter syntax and return helpful error if unsupported
pub fn validate_filter_syntax(filter: &str) -> Result<(), String> {
    let filter_lower = filter.to_lowercase();

    // Skip validation if aliases have been expanded
    if filter_lower.contains("!project:inbox") {
        return Ok(());
    }

    // Check for known unsupported patterns
    for (pattern, alternative) in UNSUPPORTED_WITH_ALTERNATIVES {
        if filter_lower.contains(pattern) {
            return Err(format!(
                "Filter syntax '{}' is not supported by Todoist API v1.\n\
                 Alternative: {}\n\
                 See https://www.todoist.com/help/articles/introduction-to-filters-V98wIH",
                pattern, alternative
            ));
        }
    }

    // ... rest of validation
}
```

**Step 4: Run tests**

Run: `cargo test expand_filter_aliases --lib`

Expected: All tests pass

**Step 5: Test manually**

```bash
# Test no project alias
cargo run -- tasks --filter 'no project' --format json | jq 'length'

# Should return same as !project:Inbox
cargo run -- tasks --filter '!project:Inbox' --format json | jq 'length'
```

Expected: Both return same number of tasks

**Step 6: Commit**

```bash
git add src/filter_validator.rs src/api.rs
git commit -m "feat: add filter alias support for common patterns

- Expands 'no project' to '!project:Inbox'
- Processes aliases before validation
- Maintains original filter string for error messages
- Makes filters more user-friendly

Users can now use intuitive filter names that map to API syntax."
```

---

### Task 4: Update Documentation

**Goal**: Document supported filter syntax and known limitations

**Files:**
- Modify: `README.md`
- Create: `docs/FILTER_SYNTAX.md`

**Step 1: Create comprehensive filter syntax guide**

Create `docs/FILTER_SYNTAX.md`:

```markdown
# Todoist Filter Syntax Support

This document describes which Todoist filter syntax is supported by todorust and how to use them effectively.

## Supported Filters

### Date Filters

| Filter | Example | Description |
|--------|---------|-------------|
| `today` | `todorust tasks --filter today` | Tasks due today |
| `tomorrow` | `todorust tasks --filter tomorrow` | Tasks due tomorrow |
| `overdue` | `todorust tasks --filter overdue` | Overdue tasks |
| `no date` | `todorust tasks --filter 'no date'` | Tasks without a date |
| `5 days` | `todorust tasks --filter '5 days'` | Tasks due in next 5 days |
| `due before:DATE` | `todorust tasks --filter 'due before:2026-01-20'` | Tasks due before date |
| `due after:DATE` | `todorust tasks --filter 'due after:today'` | Tasks due after date |
| `created:DATE` | `todorust tasks --filter 'created:today'` | Tasks created on date |

### Completed Tasks

⚠️ **Important**: Completed tasks require special handling due to Todoist API v1 limitations.

| Filter | Example | Notes |
|--------|---------|-------|
| `completed today` | `todorust tasks --filter 'completed today'` | ✅ Supported |
| `completed within "X days of today"` | `todorust tasks --filter 'completed within "7 days of today"'` | ✅ Supported (todorust extension) |

### Priority Filters

| Filter | Example | Description |
|--------|---------|-------------|
| `p1` | `toderust tasks --filter p1` | Priority 1 (highest) |
| `p2` | `todorust tasks --filter p2` | Priority 2 |
| `p3` | `todorust tasks --filter p3` | Priority 3 |
| `p4` or `no priority` | `todorust tasks --filter p4` | Priority 4 (lowest) |
| `priority:4` | `todorust tasks --filter 'priority:4'` | Alternative syntax |

⚠️ **Note**: Priority values in filter syntax are inverted from API values:
- Filter `priority:4` queries for API priority 1 (highest/red)
- Filter `priority:1` queries for API priority 4 (lowest/gray)

### Project & Section Filters

| Filter | Example | Description |
|--------|---------|-------------|
| `#ProjectName` | `todorust tasks --filter '#Work'` | Tasks in project |
| `##ProjectName` | `todorust tasks --filter '##Work'` | Tasks in project + sub-projects |
| `/SectionName` | `todorust tasks --filter '/Meetings'` | Tasks in section |
| `!project:Inbox` | `todorust tasks --filter '!project:Inbox'` | Tasks NOT in Inbox |

⚠️ **Alias**: `no project` is automatically expanded to `!project:Inbox`

### Label Filters

| Filter | Example | Description |
|--------|---------|-------------|
| `@labelname` | `todorust tasks --filter '@urgent'` | Tasks with label |
| `no labels` | `todorust tasks --filter 'no labels'` | Tasks without labels |

### Combining Filters

Use these operators to combine filters:

| Operator | Symbol | Example |
|----------|--------|---------|
| AND | `&` or space | `today & @work` |
| OR | `\|` | `today \| overdue` |
| NOT | `!` | `!project:Inbox` |
| Grouping | `()` | `(today \| overdue) & #Work` |

Examples:

```bash
# High priority work tasks due today
todorust tasks --filter 'p1 & #Work & today'

# Tasks due today or overdue
todorust tasks --filter 'today | overdue'

# Tasks in Work or Personal projects
todorust tasks --filter '#Work | #Personal'
```

## Unsupported Filters

The following filters are **NOT supported** by Todoist API v1 and will return helpful error messages:

| Filter | Reason | Alternative |
|--------|--------|-------------|
| `no project` | Not in API v1 | Use `!project:Inbox` (auto-expanded) |
| `assigned to: nobody` | Not supported | Use `!assigned` |

For complete list of supported filters, see:
https://www.todoist.com/help/articles/introduction-to-filters-V98wIH

## Examples

### Daily Review
```bash
# Show all tasks due today or overdue
todorust tasks --filter 'today | overdue' --format checklist

# Show completed tasks from today
todorust tasks --filter 'completed today' --format checklist
```

### Project Focus
```bash
# Show all high-priority tasks in Work project
todorust tasks --filter 'p1 & #Work' --format structured

# Show work tasks excluding subtasks
todorust tasks --filter '#Work & !subtask'
```

### Date Range Queries
```bash
# Tasks completed in the last 7 days
todorust tasks --filter 'completed within "7 days of today"' --format json

# Tasks due within the next week
todorust tasks --filter '7 days' --format structured
```

## Error Messages

If you use unsupported filter syntax, todorust will provide:

1. Clear error message explaining why the filter isn't supported
2. Suggested alternative syntax
3. Link to official Todoist filter documentation

Example:

```
Error: Invalid Input

Filter syntax 'no project' is not supported by Todoist API v1.
Alternative: Use '!project:Inbox' instead
See https://www.todoist.com/help/articles/introduction-to-filters-V98wIH
```

## Tips

1. **Use quotes for filters with spaces**: `'today & @work'`
2. **Test complex filters in Todoist web app first**: This ensures the syntax is valid
3. **Priority filter values are inverted**: `priority:4` = highest priority (API value 1)
4. **Completed tasks use special endpoint**: Not available in regular `/tasks/filter` endpoint
```

**Step 2: Update README with filter section**

Add to `README.md` after the Usage section:

```markdown
## Filter Syntax

`todorust` supports powerful Todoist filter syntax to query tasks:

### Quick Examples

```bash
# Tasks due today
todorust tasks --filter today

# High priority work tasks
todorust tasks --filter 'p1 & #Work'

# Completed tasks from today
todorust tasks --filter 'completed today' --format checklist

# Tasks due in the next 7 days
todorust tasks --filter '7 days'

# Tasks completed in the last week
todorust tasks --filter 'completed within "7 days of today"'
```

### Common Filters

| Filter Type | Example | Description |
|-------------|---------|-------------|
| Date | `today`, `overdue`, `5 days` | Time-based filters |
| Priority | `p1`, `priority:4` | Priority levels (p1=highest) |
| Project | `#Work`, `!project:Inbox` | Project filters |
| Label | `@urgent`, `no labels` | Label filters |
| Completed | `completed today`, `completed within "7 days of today"` | Completed tasks |

⚠️ **Important Notes:**

- Use quotes for filters with spaces: `'today & @work'`
- Priority filter `p1` = highest priority (red), `p4` = lowest (gray)
- `completed today` uses a special API endpoint
- See [Filter Syntax Documentation](docs/FILTER_SYNTAX.md) for complete reference

### Combining Filters

Use operators to combine filters:
- AND: `&` or space → `today & @work`
- OR: `\|` → `today \| overdue`
- NOT: `!` → `!project:Inbox`

For all supported filters and examples, see [docs/FILTER_SYNTAX.md](docs/FILTER_SYNTAX.md).
```

**Step 3: Verify documentation builds**

Run: `cargo doc --no-deps --open`

Expected: Documentation opens in browser successfully

**Step 4: Test examples from docs**

```bash
# Test each example from documentation
cargo run -- tasks --filter 'today & @work' --format checklist
cargo run -- tasks --filter 'completed within "7 days of today"' --format json
cargo run -- tasks --filter 'no project' --format json | jq 'length'
```

Expected: All examples work without errors

**Step 5: Commit**

```bash
git add README.md docs/FILTER_SYNTAX.md
git commit -m "docs: add comprehensive filter syntax documentation

- Created dedicated FILTER_SYNTAX.md with all supported syntax
- Added filter section to README with quick examples
- Documented priority filter inversion (p1=highest)
- Listed unsupported filters with alternatives
- Added combining filters examples
- Included error message guide

Users now have complete reference for filter syntax usage."
```

---

### Task 5: Add Integration Tests for Filter Improvements

**Goal**: Ensure all filter improvements work correctly with real API

**Files:**
- Modify: `src/api.rs` (add integration tests)

**Step 1: Add test for filter validation**

Add to `src/api.rs` tests module:

```rust
#[tokio::test]
#[ignore]
async fn test_unsupported_filter_syntax_error() {
    let client = TodoistClient::new(get_test_token());

    // Try to use unsupported syntax
    let result = client.get_tasks(Some("no project".to_string())).await;

    // Should get an InvalidInput error, not HTTP error
    assert!(result.is_err());
    match result.unwrap_err() {
        TodoError::InvalidInput(msg) => {
            assert!(msg.contains("!project:Inbox"), "Error should suggest alternative");
        }
        _ => panic!("Expected InvalidInput error"),
    }
}
```

**Step 2: Add test for date range parsing**

```rust
#[tokio::test]
#[ignore]
async fn test_completed_within_date_range() {
    let client = TodoistClient::new(get_test_token());

    // Create and complete a task
    let task = client
        .create_task("Test date range parsing", None, None, None, None)
        .await
        .unwrap();
    client.complete_task(&task.id).await.unwrap();

    // Wait for API processing
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Query with date range
    let tasks = client
        .get_tasks(Some("completed within \"7 days of today\"".to_string()))
        .await
        .unwrap();

    // Should find our completed task
    let found = tasks.iter().any(|t| t.id == task.id);
    assert!(found, "Should find completed task within 7 days");

    // Cleanup
    let _ = client.delete_task(&task.id).await;
}
```

**Step 3: Add test for filter alias expansion**

```rust
#[tokio::test]
#[ignore]
async fn test_no_project_alias_expansion() {
    let client = TodoistClient::new(get_test_token());

    // Create task outside Inbox
    let task = client
        .create_task("Test alias expansion", Some("Work".to_string()), None, None, None)
        .await
        .unwrap();

    // Query with alias
    let tasks = client
        .get_tasks(Some("no project".to_string()))
        .await
        .unwrap();

    // Should find the task (expanded to !project:Inbox)
    let found = tasks.iter().any(|t| t.id == task.id);
    assert!(found, "Should find task when using 'no project' alias");

    // Cleanup
    let _ = client.delete_task(&task.id).await;
}
```

**Step 4: Run integration tests**

Run: `cargo test --lib -- --ignored

Expected: All new integration tests pass

**Step 5: Commit**

```bash
git add src/api.rs
git commit -m "test: add integration tests for filter improvements

- Test unsupported filter syntax validation
- Test date range parsing for completed tasks
- Test filter alias expansion
- All tests validate error messages and behavior

Ensures filter improvements work correctly with real API."
```

---

## Testing Strategy

### Manual Testing Checklist

**Validation Tests:**
```bash
# Test unsupported syntax gives helpful error
cargo run -- tasks --filter 'no project' 2>&1 | grep "Alternative"
cargo run -- tasks --filter 'completed within "7 days"' 2>&1 | grep "completed today"
```

**Date Range Tests:**
```bash
# Test completed within works
cargo run -- tasks --filter 'completed within "7 days of today"' --format json | jq 'length'

# Test completed today still works
cargo run -- tasks --filter 'completed today' --format json | jq 'length'
```

**Alias Tests:**
```bash
# Test no project expansion
cargo run -- tasks --filter 'no project' --format json | jq 'length'
cargo run -- tasks --filter '!project:Inbox' --format json | jq 'length'
# Should return same count
```

### Automated Tests

```bash
# Unit tests
cargo test filter_validator --lib
cargo test parse_completed_date_range --lib
cargo test expand_filter_aliases --lib

# Integration tests
cargo test --lib -- --ignored
```

---

## Rollback Plan

If any task breaks functionality:

1. **Revert the commit**: `git revert HEAD`
2. **Identify the issue**: Add more debug logging
3. **Fix and retry**: Create new commit with fix

---

## Success Criteria

✅ Unsupported filter syntax returns helpful error messages before API call
✅ `completed within "7 days of today"` parses and works correctly
✅ `no project` alias expands to `!project:Inbox` automatically
✅ Documentation covers all supported filter syntax
✅ All unit tests pass
✅ All integration tests pass
✅ Error messages link to official documentation

---

## Notes

- Todoist API v1 has limitations on completed tasks - must use `/tasks/completed/*` endpoints
- Official filter documentation: https://www.todoist.com/help/articles/introduction-to-filters-V98wIH
- Priority values are inverted: Filter `priority:4` = API `priority:1`
- Date range parsing uses chrono crate for accurate date calculations
- Alias expansion happens before validation to prevent false errors
