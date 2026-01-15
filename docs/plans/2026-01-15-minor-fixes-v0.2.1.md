# Minor Features Fix Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Fix two minor gaps in PRD compliance: add labels support to create command, and implement output formatting for projects/filters commands.

**Architecture:** Extend existing CreateTaskRequest struct to support labels array. Implement Formattable trait for Project and Filter types to support multiple output formats. Follow existing patterns from TaskOutput formatting.

**Tech Stack:** Rust, clap (CLI), serde (JSON), existing todorust codebase

---

## Task 1: Add labels Support to Create Command

**Files:**
- Modify: `src/api.rs:220-223` (CreateTaskRequest struct)
- Modify: `src/api.rs:615-648` (create_task method signature)
- Modify: `src/main.rs:39-50` (Commands::Create enum)
- Modify: `src/main.rs:137-167` (Create command match arm)
- Test: `src/api.rs` (integration test)

**Step 1: Update CreateTaskRequest struct to add labels field**

Find the CreateTaskRequest struct in src/api.rs around line 220-223. It should look like:

```rust
struct CreateTaskRequest {
    content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    due_string: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<u8>,
}
```

Replace with:

```rust
struct CreateTaskRequest {
    content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    due_string: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<Vec<String>>,
}
```

**Step 2: Update create_task method signature**

Find the create_task method in src/api.rs around line 615. The signature should be:

```rust
pub async fn create_task(
    &self,
    content: &str,
    project_id: Option<String>,
    due_date: Option<String>,
    priority: Option<u8>,
) -> Result<TaskOutput>
```

Replace with:

```rust
pub async fn create_task(
    &self,
    content: &str,
    project_id: Option<String>,
    due_date: Option<String>,
    priority: Option<u8>,
    labels: Option<Vec<String>>,
) -> Result<TaskOutput>
```

**Step 3: Update create_task method body to pass labels**

Find where CreateTaskRequest is constructed in create_task method (around line 630-648). Add the labels field:

```rust
let request = CreateTaskRequest {
    content: content.to_string(),
    project_id,
    due_string: due_date,
    priority,
    labels,  // Add this line
};
```

**Step 4: Add labels parameter to CLI Commands enum**

In src/main.rs, find the Create command enum variant around line 39-50. Add labels field:

```rust
Create {
    #[arg(long)]
    content: String,
    #[arg(long)]
    project_id: Option<String>,
    #[arg(long)]
    due_date: Option<String>,
    #[arg(long)]
    priority: Option<u8>,
    #[arg(long)]
    labels: Option<String>,  // Add this line
},
```

**Step 5: Parse labels in Create command handler**

In src/main.rs, find the Create command match arm around line 137-167. After priority validation, add labels parsing:

Find this code:
```rust
if let Some(p) = priority {
    if !validate_priority(p) {
        return Err(TodoError::InvalidInput(
            "Priority must be between 1 and 4".to_string(),
        ));
    }
}

let task = client
    .create_task(&content, project_id, due_date, priority)
    .await?;
```

Replace with:
```rust
if let Some(p) = priority {
    if !validate_priority(p) {
        return Err(TodoError::InvalidInput(
            "Priority must be between 1 and 4".to_string(),
        ));
    }
}

// Parse labels from comma-separated string
let labels_vec = labels.and_then(|l| {
    if l.is_empty() {
        None
    } else {
        Some(l.split(',').map(|s| s.trim().to_string()).collect())
    }
});

let task = client
    .create_task(&content, project_id, due_date, priority, labels_vec)
    .await?;
```

**Step 6: Add integration test for labels**

In src/api.rs, find test_create_task_real around line 615. Add a new test after it:

```rust
#[tokio::test]
#[ignore]
async fn test_create_task_with_labels_real() {
    use crate::Formattable;
    let client = TodoistClient::new(get_test_token());

    let task = client
        .create_task(
            "Test task with labels",
            None,
            None,
            None,
            Some(vec!["test-label".to_string(), "urgent".to_string()]),
        )
        .await
        .unwrap();

    assert_eq!(task.content, "Test task with labels");
    assert!(!task.labels.is_empty());

    // Cleanup
    let _ = client.delete_task(&task.id).await;
}
```

**Step 7: Run the new integration test**

Run: `cargo test test_create_task_with_labels_real -- --ignored`
Expected: PASS (task created with labels)

**Step 8: Test manually with CLI**

Run: `./target/debug/todorust create --content "Test labels" --labels "test,urgent"`
Expected: Task created successfully

Verify: Check Todoist app or run:
```bash
./target/debug/todorust tasks --filter 'test labels' --format checklist
```

**Step 9: Commit**

```bash
git add src/api.rs src/main.rs
git commit -m "feat: add labels support to create command

Add --labels parameter to create command for adding labels to tasks.
Labels are specified as comma-separated values.

Usage:
  todorust create --content "Buy milk" --labels "urgent,shopping"

Resolves part of PRD Gap #2 - labels support in create task.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 2: Implement Formattable for Projects

**Files:**
- Modify: `src/formatter.rs` (add Project formatting)
- Modify: `src/lib.rs` (export Project)
- Modify: `src/main.rs:122-129` (Projects command handler)
- Test: `src/formatter.rs` (unit tests)

**Step 1: Update lib.rs to export Project**

In src/lib.rs, find the exports section. Add Project to the exports:

Current:
```rust
pub mod api;
pub mod config;
pub mod error;
pub mod formatter;
pub mod models;

pub use formatter::{OutputFormat, Formattable};
```

Change to:
```rust
pub mod api;
pub mod config;
pub mod error;
pub mod formatter;
pub mod models;

pub use formatter::{OutputFormat, Formattable};
pub use models::Project;  // Add this line
```

**Step 2: Implement Formattable for Vec<Project>**

In src/formatter.rs, after the Formattable impl for Vec<TaskOutput> (around line 50), add:

```rust
impl Formattable for Vec<Project> {
    fn format(&self, format: &OutputFormat) -> String {
        match format {
            OutputFormat::Json => format_json_projects(self),
            OutputFormat::Checklist => format_projects_checklist(self),
            OutputFormat::Structured => format_projects_structured(self),
        }
    }
}

fn format_json_projects(projects: &[Project]) -> String {
    serde_json::to_string_pretty(projects).unwrap_or_default()
}

fn format_projects_checklist(projects: &[Project]) -> String {
    projects.iter()
        .map(|project| {
            let indicator = if project.is_favorite { "⭐ " } else { "" };
            format!("- [ ] {}{}{}", indicator, project.name,
                    if project.is_shared { " (shared)" } else { "" })
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_projects_structured(projects: &[Project]) -> String {
    projects.iter()
        .map(|project| {
            let meta = vec![
                if project.is_favorite { Some("⭐ Favorite".to_string()) } else { None },
                if project.is_shared { Some("👥 Shared".to_string()) } else { None },
            ]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
            .join(" | ");

            format!(
                "### {}\n\n**Color:** {}\n**ID:** {}\n{}",
                project.name,
                project.color,
                project.id,
                if meta.is_empty() { String::new() } else { format!("**Meta:** {}\n", meta) }
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}
```

**Step 3: Add unit tests for Project formatting**

In src/formatter.rs, in the tests module (after the TaskOutput tests), add:

```rust
#[test]
fn test_format_projects_checklist() {
    let projects = vec![
        Project {
            id: "1".to_string(),
            name: "Work".to_string(),
            color: "blue".to_string(),
            is_shared: false,
            is_favorite: true,
        },
        Project {
            id: "2".to_string(),
            name: "Personal".to_string(),
            color: "green".to_string(),
            is_shared: true,
            is_favorite: false,
        },
    ];

    let output = projects.format(&OutputFormat::Checklist);
    assert!(output.contains("- [ ] ⭐ Work"));
    assert!(output.contains("- [ ] Personal (shared)"));
}

#[test]
fn test_format_projects_structured() {
    let projects = vec![Project {
        id: "1".to_string(),
        name: "Work".to_string(),
        color: "blue".to_string(),
        is_shared: false,
        is_favorite: true,
    }];

    let output = projects.format(&OutputFormat::Structured);
    assert!(output.contains("### Work"));
    assert!(output.contains("**Color:** blue"));
    assert!(output.contains("⭐ Favorite"));
}

#[test]
fn test_format_projects_json() {
    let projects = vec![Project {
        id: "1".to_string(),
        name: "Test".to_string(),
        color: "red".to_string(),
        is_shared: false,
        is_favorite: false,
    }];

    let output = projects.format(&OutputFormat::Json);
    assert!(output.contains("\"name\""));
    assert!(output.contains("Test"));
}
```

**Step 4: Update Projects command handler to use formatter**

In src/main.rs, find the Projects command handler around line 122-129. It should currently be:

```rust
Commands::Projects { format } => {
    let output_format = format.unwrap_or(cli.format);
    let projects = client.get_projects().await?;
    // TODO: Implement project formatting
    let output = serde_json::to_string_pretty(&projects)?;
    println!("{}", output);
    Ok(())
}
```

Replace with:
```rust
Commands::Projects { format } => {
    let output_format = format.unwrap_or(cli.format);
    let projects = client.get_projects().await?;
    let output = projects.format(&output_format);
    println!("{}", output);
    Ok(())
}
```

**Step 5: Run unit tests**

Run: `cargo test formatter::tests::test_format_projects`
Expected: All 3 new tests PASS

**Step 6: Test manually with CLI**

Run: `./target/debug/todorust projects --format checklist`
Expected: Checklist of projects with favorites marked

Run: `./target/debug/todorust projects --format structured`
Expected: Structured Markdown with project details

**Step 7: Commit**

```bash
git add src/formatter.rs src/lib.rs src/main.rs
git commit -m "feat: add output format support for projects command

Implement Formattable trait for Vec<Project> supporting:
- JSON format (default)
- Checklist format with favorites indicator
- Structured format with detailed project metadata

Updates Projects command to use --format parameter.

Resolves part of PRD Gap #3 - Projects format output.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 3: Implement Formattable for Filters

**Files:**
- Modify: `src/formatter.rs` (add Filter formatting)
- Modify: `src/main.rs:130-137` (Filters command handler)
- Test: `src/formatter.rs` (unit tests)

**Step 1: Implement Formattable for Vec<Filter>**

In src/formatter.rs, after the Project impl (around line 150), add:

```rust
impl Formattable for Vec<Filter> {
    fn format(&self, format: &OutputFormat) -> String {
        match format {
            OutputFormat::Json => format_json_filters(self),
            OutputFormat::Checklist => format_filters_checklist(self),
            OutputFormat::Structured => format_filters_structured(self),
        }
    }
}

fn format_json_filters(filters: &[Filter]) -> String {
    serde_json::to_string_pretty(filters).unwrap_or_default()
}

fn format_filters_checklist(filters: &[Filter]) -> String {
    filters.iter()
        .map(|filter| {
            format!("- [ ] {} ({})", filter.name, filter.query)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_filters_structured(filters: &[Filter]) -> String {
    filters.iter()
        .map(|filter| {
            format!(
                "### {}\n\n**Filter:** `{}`\n**ID:** {}\n",
                filter.name, filter.query, filter.id
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}
```

**Step 2: Add unit tests for Filter formatting**

In src/formatter.rs, in the tests module after the Project tests, add:

```rust
#[test]
fn test_format_filters_checklist() {
    let filters = vec![
        Filter {
            id: "1".to_string(),
            name: "This Week".to_string(),
            query: "due within \"7 days of today\"".to_string(),
        },
        Filter {
            id: "2".to_string(),
            name: "Work High Priority".to_string(),
            query: "project:Work & priority:4".to_string(),
        },
    ];

    let output = filters.format(&OutputFormat::Checklist);
    assert!(output.contains("- [ ] This Week"));
    assert!(output.contains("(due within \"7 days of today\")"));
}

#[test]
fn test_format_filters_structured() {
    let filters = vec![Filter {
        id: "1".to_string(),
        name: "Urgent Tasks".to_string(),
        query: "priority:4".to_string(),
    }];

    let output = filters.format(&OutputFormat::Structured);
    assert!(output.contains("### Urgent Tasks"));
    assert!(output.contains("**Filter:** `priority:4`"));
    assert!(output.contains("**ID:** 1"));
}

#[test]
fn test_format_filters_json() {
    let filters = vec![Filter {
        id: "1".to_string(),
        name: "Test".to_string(),
        query: "project:Work".to_string(),
    }];

    let output = filters.format(&OutputFormat::Json);
    assert!(output.contains("\"name\""));
    assert!(output.contains("Test"));
}
```

**Step 3: Update Filters command handler to use formatter**

In src/main.rs, find the Filters command handler around line 130-137. It should currently be:

```rust
Commands::Filters { format } => {
    let output_format = format.unwrap_or(cli.format);
    let filters = client.get_filters().await?;
    // TODO: Implement filter formatting
    let output = serde_json::to_string_pretty(&filters)?;
    println!("{}", output);
    Ok(())
}
```

Replace with:
```rust
Commands::Filters { format } => {
    let output_format = format.unwrap_or(cli.format);
    let filters = client.get_filters().await?;
    let output = filters.format(&output_format);
    println!("{}", output);
    Ok(())
}
```

**Step 4: Run unit tests**

Run: `cargo test formatter::tests::test_format_filters`
Expected: All 3 new tests PASS

**Step 5: Test manually with CLI**

Run: `./target/debug/todorust filters --format checklist`
Expected: Checklist of filters with query expressions

Run: `./target/debug/todorust filters --format structured`
Expected: Structured Markdown with filter details

**Step 6: Commit**

```bash
git add src/formatter.rs src/main.rs
git commit -m "feat: add output format support for filters command

Implement Formattable trait for Vec<Filter> supporting:
- JSON format (default)
- Checklist format with filter queries
- Structured format with detailed filter metadata

Updates Filters command to use --format parameter.

Resolves PRD Gap #3 - Filters format output.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 4: Update Documentation

**Files:**
- Modify: `README.md`
- Modify: `docs/prd-gap-analysis.md` (update status)

**Step 1: Update README with labels usage**

In README.md, find the "Create Task" section (around line 56). Add labels example:

Current:
```markdown
### Create Task

```bash
# Basic task
todorust create --content "Buy milk"

# With project and due date
todorust create --content "Write report" --project-id "123" --due-date "2026-01-20" --priority 4
```
```

Replace with:
```markdown
### Create Task

```bash
# Basic task
todorust create --content "Buy milk"

# With project and due date
todorust create --content "Write report" --project-id "123" --due-date "2026-01-20" --priority 4

# With labels
todorust create --content "Urgent task" --labels "urgent,work"
```
```

**Step 2: Update README with Projects/Filters formats**

Find the "Output Formats" section. Add examples for Projects and Filters after the Tasks examples (around line 84):

```markdown
### Get Projects

```bash
# Get projects as JSON (default)
todorust projects

# Get projects as checklist
todorust projects --format checklist

# Get projects with detailed info
todorust projects --format structured
```

### Get Filters

```bash
# Get filters as JSON (default)
todorust filters

# Get filters as checklist
todorust filters --format checklist

# Get filters with detailed info
todorust filters --format structured
```
```

**Step 3: Update gap analysis status**

In docs/prd-gap-analysis.md, update the gap sections:

Find Gap #2 section and change:
```markdown
**❌ Gap #2: 创建任务缺少 labels/tags 参数支持**
```

To:
```markdown
**✅ Gap #2: 已修复** (v0.2.1)
```

Find Gap #3 section and change:
```markdown
**⚠️ Gap #3: Projects 和 Filters 不支持多格式输出**
```

To:
```markdown
**✅ Gap #3: 已修复** (v0.2.1)
```

**Step 4: Run all tests to verify**

Run: `cargo test`
Expected: All tests pass (should be 39 tests: 21 unit + 9 integration + 9 new formatter tests)

**Step 5: Test end-to-end scenarios**

Test labels:
```bash
./target/debug/todorust create --content "Test labels feature" --labels "test,v0.2.1"
./target/debug/todorust tasks --format checklist | grep "Test labels feature"
```

Test projects formats:
```bash
./target/debug/todorust projects --format checklist
./target/debug/todorust projects --format structured
```

Test filters formats:
```bash
./target/debug/todorust filters --format checklist
./target/debug/todorust filters --format structured
```

**Step 6: Update version**

In Cargo.toml, change version from "0.2.0" to "0.2.1":

```toml
[package]
name = "todorust"
version = "0.2.1"
```

**Step 7: Commit**

```bash
git add README.md docs/prd-gap-analysis.md Cargo.toml
git commit -m "docs: update documentation for v0.2.1

Update README with:
- Labels parameter usage examples
- Projects/Filters output format examples

Update gap analysis:
- Mark Gap #2 and #3 as resolved
- All PRD MVP requirements now met

Bump version to 0.2.1

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 5: Final Testing and Release

**Files:**
- Create: `.gitignore` (verify exists)

**Step 1: Run complete test suite**

Run: `cargo test --all`
Expected: All 39 tests pass (21 unit + 9 integration + 9 new formatter tests)

**Step 2: Run integration tests**

Run: `cargo test -- --ignored`
Expected: All 10 integration tests pass (8 original + 1 labels + 1 existing)

**Step 3: Verify build**

Run: `cargo build --release`
Expected: Clean release build

**Step 4: Manual testing checklist**

Test all new features:

```bash
# Test labels
./target/release/todorust create --content "Release test" --labels "test,v0.2.1" --format checklist

# Test projects formats
./target/release/todorust projects --format checklist
./target/release/todorust projects --format structured

# Test filters formats
./target/release/todorust filters --format checklist
./target/release/todorust filters --format structured

# Test global format
./target/release/todorust --format structured projects
```

Expected: All commands work correctly with proper output

**Step 5: Tag release**

Run: `git tag v0.2.1`

Verify tag:
Run: `git tag -l | grep v0.2.1`

**Step 6: Commit**

```bash
git add .gitignore  # if needed
git commit -m "chore: prepare for v0.2.1 release

All tests passing (39 tests)
All PRD MVP requirements met:
- ✅ Labels support in create command
- ✅ Projects/Filters output formatting
- ✅ Complete Obsidian workflow support

Ready for release.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Summary

This plan addresses 2 of the 3 PRD gaps:

**Fixes Included:**
- ✅ Gap #2: Add labels support to create command
- ✅ Gap #3: Projects/Filters output formatting

**Not Included (Low Priority):**
- ⏭️ Gap #1: Pagination support (deferred to future based on user feedback)

**Test Coverage:**
- 9 new unit tests (3 for Projects, 3 for Filters, 3 for labels)
- 1 new integration test (labels)
- Total: 40 tests (21 unit + 10 integration + 9 formatter)

**Estimated Effort:** 2-3 hours

**Success Criteria:**
- ✅ All PRD MVP requirements met
- ✅ All commands support --format parameter
- ✅ Labels can be added to tasks via CLI
- ✅ All tests passing
- ✅ Documentation updated

**Post-Implementation:**
- PRD gap analysis will show 100% MVP compliance
- Ready for v0.2.1 release
- Only Gap #1 (pagination) remains as Phase 2 feature
