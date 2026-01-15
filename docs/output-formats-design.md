# Output Formats Design

## Overview

Add `--format` parameter to todorust CLI to support multiple output formats for different use cases.

## Formats

### 1. JSON (Default)

**Purpose:** Programmatic access, API integration, data processing

**Usage:**
```bash
todorust tasks
todorust tasks --format json
```

**Output:** Existing JSON format (no change)

```json
[
  {
    "id": "123",
    "content": "Task name",
    "project_id": "456",
    "project_name": "Work",
    "due_date": "2026-01-15",
    "is_completed": true,
    "priority": 4,
    "labels": ["urgent"]
  }
]
```

### 2. Checklist

**Purpose:** Obsidian daily notes, simple task lists

**Usage:**
```bash
todorust tasks --format checklist
todorust tasks --filter 'completed today' --format checklist
```

**Output:** Markdown checklist format

```markdown
- [x] Task name (Project)
- [ ] Another task (Work)
- [x] Third task (Personal)
```

**Rules:**
- Completed tasks: `- [x]`
- Incomplete tasks: `- [ ]`
- Show project name in parentheses
- Omit parentheses if no project
- One task per line
- No additional metadata

### 3. Structured

**Purpose:** Weekly reports, project summaries, organized views

**Usage:**
```bash
todorust tasks --format structured
todorust tasks --filter 'completed within "7 days"' --format structured
```

**Output:** Markdown structured by project

```markdown
## Work

- [x] Complete project proposal (Priority: 4)
- [ ] Review documentation (Priority: 3)

## Personal

- [x] Buy groceries
- [ ] Schedule dentist appointment

## Inbox

- [ ] Process new tasks
```

**Rules:**
- Group tasks by project_name
- Level 2 heading for project name (##)
- Show priority for Work/Personal projects if > 1
- Empty tasks list for projects with no tasks
- Sort projects alphabetically
- "Inbox" project shown if tasks have no project

## Implementation

### CLI Changes

**Add global `--format` parameter:**

```rust
#[derive(Parser)]
#[command(name = "todorust")]
struct Cli {
    #[arg(long, short, global = true, default_value = "json")]
    format: OutputFormat,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Clone, Debug, ValueEnum)]
enum OutputFormat {
    Json,
    Checklist,
    Structured,
}
```

**Pass format to commands:**

```rust
#[derive(Subcommand)]
enum Commands {
    Tasks {
        #[arg(long)]
        filter: Option<String>,
        #[arg(long, short)]
        format: Option<OutputFormat>,  // Override global format
    },
    // ... other commands
}
```

### Output Formatting

**New module: `src/formatter.rs`**

```rust
pub enum OutputFormat {
    Json,
    Checklist,
    Structured,
}

pub trait Formattable {
    fn format(&self, format: &OutputFormat) -> String;
}

impl Formattable for Vec<TaskOutput> {
    fn format(&self, format: &OutputFormat) -> String {
        match format {
            OutputFormat::Json => serde_json::to_string_pretty(self).unwrap(),
            OutputFormat::Checklist => format_checklist(self),
            OutputFormat::Structured => format_structured(self),
        }
    }
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
    let grouped = tasks.iter()
        .fold(std::collections::HashMap::new(), |mut acc, task| {
            let project = task.project_name.as_deref()
                .unwrap_or("Inbox")
                .to_string();
            acc.entry(project).or_insert_with(Vec::new).push(task);
            acc
        });

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

### Update Commands

**Modify main.rs:**

```rust
match cli.command {
    Commands::Tasks { filter, format } => {
        let output_format = format.or(cli.format);
        let tasks = client.get_tasks(filter).await?;
        let output = tasks.format(&output_format);
        println!("{}", output);
    },
    Commands::Projects { format } => {
        let output_format = format.or(cli.format);
        let projects = client.get_projects().await?;
        let output = projects.format(&output_format);
        println!("{}", output);
    },
    // ... similar for other commands
}
```

## Examples

### Daily Report

```bash
$ todorust tasks --filter 'completed today' --format checklist
- [x] Complete project proposal (Work)
- [x] Review PRs (Work)
- [x] Buy groceries (Personal)
```

### Weekly Report

```bash
$ todorust tasks --filter 'completed within "7 days"' --format structured
## Personal

- [x] Buy groceries
- [x] Schedule dentist appointment
- [ ] Pay bills

## Work

- [x] Complete project proposal (Priority: 4)
- [ ] Review documentation (Priority: 3)
- [x] Team meeting (Priority: 4)

## Learning

- [ ] Read Rust chapter 5 (Priority: 2)
```

### Programmatic Use

```bash
$ todorust tasks --filter 'project:Work' | jq '.[] | .content'
Complete project proposal
Review documentation
Team meeting
```

## Backward Compatibility

- **Default format is JSON** - existing scripts and integrations continue to work
- **Optional parameter** - users who don't specify `--format` get JSON
- **No breaking changes** - all existing functionality preserved

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn mock_tasks() -> Vec<TaskOutput> {
        vec![
            TaskOutput {
                id: "1".to_string(),
                content: "Task 1".to_string(),
                project_id: Some("p1".to_string()),
                project_name: Some("Work".to_string()),
                is_completed: true,
                priority: 4,
                labels: vec![],
                // ... other fields
            },
            // ... more tasks
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
    }

    #[test]
    fn test_format_json() {
        let tasks = mock_tasks();
        let output = tasks.format(&OutputFormat::Json);
        assert!(output.contains("\"content\""));
    }
}
```

### Integration Tests

```rust
#[tokio::test]
#[ignore]
async fn test_checklist_output_real() {
    let client = TodoistClient::new(get_test_token());
    let tasks = client.get_tasks(Some("completed today".to_string())).await.unwrap();
    let output = tasks.format(&OutputFormat::Checklist);

    // Verify format
    assert!(output.lines().all(|line| {
        line.starts_with("- [x]") || line.starts_with("- [ ]")
    }));
}
```

## Future Enhancements

### Additional Formats

- **table** - Markdown table format
- **csv** - CSV format for spreadsheet import
- **html** - HTML list format

### Custom Templates

```bash
todorust tasks --format template --template-file my-template.md
```

### Filtering in Output

- Include/exclude fields in JSON output
- Custom field separators in checklist
- Group by labels, priority, or date

## Documentation Updates

- Update README.md with format examples
- Update USAGE.md with --format parameter documentation
- Add examples to obsidian-todorust skill
- Remove JSON parsing instructions from skill (no longer needed)
