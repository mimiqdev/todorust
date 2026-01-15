# Implementation Plan: Add Output Formats Support

**Version:** v0.2.0
**Date:** 2026-01-15
**Status:** Draft

## Overview

Add `--format` parameter to support multiple output formats (JSON, Markdown checklist, Markdown structured) for better integration with Obsidian and other tools.

## Goals

1. Add `--format` global parameter with options: `json`, `checklist`, `structured`
2. Implement formatter module for output conversion
3. Update all commands to support format parameter
4. Maintain backward compatibility (JSON as default)
5. Update tests to cover all formats
6. Update documentation

## Technical Approach

### Phase 1: Core Formatter Module

**Task 1.1: Create formatter module**

Files:
- Create: `src/formatter.rs`
- Modify: `src/lib.rs`

**Steps:**

1. Create `OutputFormat` enum
```rust
#[derive(Clone, Debug, PartialEq, ValueEnum)]
pub enum OutputFormat {
    Json,
    Checklist,
    Structured,
}
```

2. Create formatter trait
```rust
pub trait Formattable {
    fn format(&self, format: &OutputFormat) -> String;
}
```

3. Implement for `Vec<TaskOutput>`
```rust
impl Formattable for Vec<TaskOutput> {
    fn format(&self, format: &OutputFormat) -> String {
        match format {
            OutputFormat::Json => format_json(self),
            OutputFormat::Checklist => format_checklist(self),
            OutputFormat::Structured => format_structured(self),
        }
    }
}
```

4. Implement format functions
```rust
fn format_json(tasks: &[TaskOutput]) -> String {
    serde_json::to_string_pretty(tasks).unwrap_or_default()
}

fn format_checklist(tasks: &[TaskOutput]) -> String {
    tasks.iter()
        .map(|task| {
            let checkbox = if task.is_completed { "[x]" } else { "[ ]" };
            let project = task.project_name.as_ref()
                .map(|p| format!(" ({})", p))
                .unwrap_or_default();
            format!("- {} {}{}", checkbox, task.content, project)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_structured(tasks: &[TaskOutput]) -> String {
    let mut grouped: std::collections::HashMap<&str, Vec<&TaskOutput>> =
        std::collections::HashMap::new();

    for task in tasks {
        let project = task.project_name.as_deref().unwrap_or("Inbox");
        grouped.entry(project).or_insert_with(Vec::new).push(task);
    }

    let mut projects: Vec<_> = grouped.into_iter().collect();
    projects.sort_by(|a, b| a.0.cmp(&b.0));

    projects.iter()
        .map(|(project, tasks)| {
            let tasks_str = tasks.iter()
                .map(|task| {
                    let checkbox = if task.is_completed { "[x]" } else { "[ ]" };
                    let priority = if task.priority > 1 {
                        format!(" (Priority: {})", task.priority)
                    } else {
                        String::new()
                    };
                    format!("- {} {}{}", checkbox, task.content, priority)
                })
                .collect::<Vec<_>>()
                .join("\n");
            format!("## {}\n\n{}", project, tasks_str)
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}
```

5. Add to lib.rs
```rust
pub mod formatter;

pub use formatter::{OutputFormat, Formattable};
```

**Testing:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn mock_tasks() -> Vec<TaskOutput> {
        vec![
            TaskOutput {
                id: "1".to_string(),
                content: "Task 1".to_string(),
                project_name: Some("Work".to_string()),
                is_completed: true,
                priority: 4,
                labels: vec![],
                project_id: Some("p1".to_string()),
                due_date: None,
                created_at: "2026-01-10T10:00:00Z".to_string(),
                order: 1,
            },
        ]
    }

    #[test]
    fn test_format_checklist() {
        let tasks = mock_tasks();
        let output = tasks.format(&OutputFormat::Checklist);
        assert!(output.contains("- [x] Task 1 (Work)"));
    }

    #[test]
    fn test_format_structured() {
        let tasks = mock_tasks();
        let output = tasks.format(&OutputFormat::Structured);
        assert!(output.contains("## Work"));
        assert!(output.contains("- [x] Task 1"));
        assert!(output.contains("(Priority: 4)"));
    }
}
```

**Commit:**
```bash
git add src/formatter.rs src/lib.rs
git commit -m "feat: add formatter module with output format support"
```

---

### Phase 2: CLI Integration

**Task 2.1: Add global --format parameter**

Files:
- Modify: `src/main.rs`

**Steps:**

1. Update Cli struct
```rust
#[derive(Parser)]
#[command(name = "todorust")]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(long, short, global = true, default_value = "json")]
    format: OutputFormat,

    #[command(subcommand)]
    command: Commands,
}
```

2. Update Commands enum to support per-command format override
```rust
#[derive(Subcommand)]
enum Commands {
    /// Get tasks with optional filter
    Tasks {
        #[arg(long)]
        filter: Option<String>,
        #[arg(long, short)]
        format: Option<OutputFormat>,
    },
    /// Get all projects
    Projects {
        #[arg(long, short)]
        format: Option<OutputFormat>,
    },
    /// Get custom filters
    Filters {
        #[arg(long, short)]
        format: Option<OutputFormat>,
    },
    /// Create a new task
    Create {
        #[arg(long)]
        content: String,
        #[arg(long)]
        project_id: Option<String>,
        #[arg(long)]
        due_date: Option<String>,
        #[arg(long)]
        priority: Option<u8>,
        #[arg(long, short)]
        format: Option<OutputFormat>,
    },
    /// Complete a task
    Complete {
        #[arg(long)]
        task_id: String,
    },
    /// Reopen a task
    Reopen {
        #[arg(long)]
        task_id: String,
    },
    /// Initialize configuration
    Init {
        #[arg(long)]
        api_token: String,
    },
}
```

3. Update main function to use formatter
```rust
#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if let Commands::Init { api_token } = cli.command {
        if let Err(e) = init_config(&api_token) {
            handle_error(e);
        }
        println!("Configuration initialized successfully!");
        return;
    }

    let result = async {
        let config = load_config()?;
        let client = TodoistClient::new(config.api_token);

        match cli.command {
            Commands::Tasks { filter, format } => {
                let output_format = format.unwrap_or(cli.format);
                let tasks = client.get_tasks(filter).await?;
                let output = tasks.format(&output_format);
                println!("{}", output);
                Ok::<(), TodoError>(())
            }
            Commands::Projects { format } => {
                let output_format = format.unwrap_or(cli.format);
                let projects = client.get_projects().await?;
                // TODO: Implement project formatting
                let output = serde_json::to_string_pretty(&projects)?;
                println!("{}", output);
                Ok(())
            }
            Commands::Filters { format } => {
                let output_format = format.unwrap_or(cli.format);
                let filters = client.get_filters().await?;
                // TODO: Implement filter formatting
                let output = serde_json::to_string_pretty(&filters)?;
                println!("{}", output);
                Ok(())
            }
            Commands::Create { content, project_id, due_date, priority, format } => {
                let output_format = format.unwrap_or(cli.format);
                // content validation
                if content.trim().is_empty() {
                    return Err(TodoError::InvalidInput(
                        "Task content cannot be empty".to_string(),
                    ));
                }
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
                // For single task, wrap in vec for formatting
                let output = vec![task].format(&output_format);
                println!("{}", output);
                Ok(())
            }
            Commands::Complete { task_id } => {
                client.complete_task(&task_id).await?;
                println!("Task {} completed", task_id);
                Ok(())
            }
            Commands::Reopen { task_id } => {
                client.reopen_task(&task_id).await?;
                println!("Task {} reopened", task_id);
                Ok(())
            }
            Commands::Init { .. } => unreachable!(),
        }
    };

    if let Err(e) = result.await {
        handle_error(e);
    }
}
```

**Testing:**
```bash
# Test default (JSON)
cargo run -- tasks

# Test checklist
cargo run -- tasks --format checklist

# Test structured
cargo run -- tasks --format structured

# Test global format
cargo run -- --format checklist tasks
```

**Commit:**
```bash
git add src/main.rs
git commit -m "feat: add --format parameter to CLI"
```

---

### Phase 3: Integration Tests

**Task 3.1: Add formatter integration tests**

Files:
- Modify: `src/api.rs`

**Steps:**

1. Add real API integration tests for formats
```rust
#[tokio::test]
#[ignore]
async fn test_checklist_format_real() {
    let client = TodoistClient::new(get_test_token());
    let tasks = client.get_tasks(Some("completed today".to_string())).await.unwrap();
    let output = tasks.format(&OutputFormat::Checklist);

    // Verify all lines are checklist items
    for line in output.lines() {
        assert!(line.starts_with("- [x]") || line.starts_with("- [ ]"),
                "Line should be checklist item: {}", line);
    }
}

#[tokio::test]
#[ignore]
async fn test_structured_format_real() {
    let client = TodoistClient::new(get_test_token());
    let tasks = client.get_tasks(None).await.unwrap();
    let output = tasks.format(&OutputFormat::Structured);

    // Verify has project headings
    assert!(output.contains("## ") || output.is_empty());

    // Verify tasks under projects
    for line in output.lines() {
        if line.starts_with("- ") {
            assert!(line.contains("[x]") || line.contains("[ ]"),
                    "Task line should have checkbox: {}", line);
        }
    }
}
```

**Testing:**
```bash
cargo test test_checklist_format_real -- --ignored
cargo test test_structured_format_real -- --ignored
```

**Commit:**
```bash
git add src/api.rs
git commit -m "test: add integration tests for output formats"
```

---

### Phase 4: Documentation Updates

**Task 4.1: Update README**

Files:
- Modify: `README.md`

**Add usage examples:**
```markdown
## Usage

### Output Formats

Todorust supports multiple output formats via the `--format` parameter:

**JSON (default):**
```bash
todorust tasks
todorust tasks --format json
```

**Markdown Checklist:**
```bash
todorust tasks --format checklist
todorust tasks --filter 'completed today' --format checklist
```

Output:
```markdown
- [x] Complete proposal (Work)
- [ ] Review docs (Work)
- [x] Buy groceries (Personal)
```

**Structured Markdown (by project):**
```bash
todorust tasks --format structured
todorust tasks --filter 'completed within "7 days"' --format structured
```

Output:
```markdown
## Personal

- [x] Buy groceries
- [ ] Pay bills

## Work

- [x] Complete proposal (Priority: 4)
- [ ] Review docs (Priority: 3)
```

### Get Tasks

```bash
# Get all tasks (JSON)
todorust tasks

# Get tasks as checklist
todorust tasks --format checklist

# Filter with custom format
todorust tasks --filter "project:Work" --format structured
```
```

**Commit:**
```bash
git add README.md
git commit -m "docs: update README with output format examples"
```

---

**Task 4.2: Update obsidian-todorust skill**

Files:
- Modify: `skills/obsidian-todorust/SKILL.md`

**Remove JSON parsing sections, simplify to:**

```markdown
## Quick Start

### Basic Usage

**Get today's completed tasks as checklist:**
```bash
todorust tasks --filter 'completed today' --format checklist
```

**Get this week's tasks organized by project:**
```bash
todorust tasks --filter 'completed within "7 days of today"' --format structured
```

**Get JSON for programmatic use:**
```bash
todorust tasks --filter 'completed today'
```

## Output Formats

### Checklist Format

Simple Markdown checklist, perfect for daily notes:
```bash
todorust tasks --format checklist
```

Output:
```
- [x] Task name (Project)
- [ ] Another task (Project)
```

### Structured Format

Organized by project with priority info:
```bash
todorust tasks --format structured
```

Output:
```
## Project Name

- [x] Task 1 (Priority: 4)
- [ ] Task 2 (Priority: 2)
```

### JSON Format

Full data structure for scripts (default):
```bash
todorust tasks
```

## Integration Examples

### Daily Notes Skill

```bash
# Get today's completed tasks as ready-to-use checklist
todorust tasks --filter 'completed today' --format checklist
```

No parsing needed - direct output to note!

### Weekly Review Skill

```bash
# Get this week's tasks organized by project
todorust tasks --filter 'completed within "7 days of today"' --format structured
```

Ready to paste into weekly review template!

### Custom Date Range

```bash
START_DATE="2026-01-10"
END_DATE="2026-01-16"
todorust tasks --filter "completed within \"$START_DATE to $END_DATE\"" --format structured
```
```

**Commit:**
```bash
git add skills/obsidian-todorust/SKILL.md
git commit -m "docs: simplify skill by removing JSON parsing (CLI handles formatting)"
```

---

### Phase 5: Final Testing and Release

**Task 5.1: Run complete test suite**

```bash
# Unit tests
cargo test

# Integration tests
cargo test -- --ignored

# Manual testing
cargo run -- tasks --format checklist
cargo run -- tasks --format structured
cargo run -- tasks --filter 'completed today' --format checklist
```

**Task 5.2: Update version**

Update `Cargo.toml`:
```toml
version = "0.2.0"
```

**Commit:**
```bash
git add Cargo.toml
git commit -m "chore: bump version to 0.2.0"
```

**Task 5.3: Create release**

```bash
git tag v0.2.0
git push origin main --tags
```

---

## Success Criteria

✅ All commands support `--format` parameter
✅ Three formats working: json, checklist, structured
✅ Backward compatible (JSON default)
✅ All tests passing
✅ Documentation updated
✅ Skill simplified (no JSON parsing needed)
✅ Integration tests pass with real API

## Estimated Effort

- Phase 1: 2-3 hours
- Phase 2: 2-3 hours
- Phase 3: 1-2 hours
- Phase 4: 1-2 hours
- Phase 5: 1 hour

**Total:** 7-11 hours

## Dependencies

- None (pure Rust implementation)

## Risks

- **Low risk** - Feature addition, no breaking changes
- **Test coverage** - Need comprehensive tests for each format
- **Documentation** - Must update all examples and skill

## Rollback Plan

If issues arise:
1. Feature can be disabled by removing `--format` from CLI
2. JSON output remains fully functional
3. Revert to previous version tag
