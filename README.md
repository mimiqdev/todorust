# Todorust CLI

![CI](https://github.com/mimiqdev/todorust/actions/workflows/ci.yml/badge.svg)
![Coverage](https://codecov.io/gh/mimiqdev/todorust/branch/main/graph/badge.svg)

A Rust CLI tool for Todoist Sync API integration with simplified JSON output for AI tools and automation workflows.

## Features

- Get tasks with full Todoist filter syntax support
- Get projects and custom filters
- Create, complete, and reopen tasks
- Clean JSON output optimized for parsing
- Simple configuration management

## Installation

```bash
cargo install --path .
```

## Command Structure

Todorust uses a **verb-resource** command pattern for intuitive CLI usage.

### Available Commands

| Command | Description |
|---------|-------------|
| `init` | Initialize configuration with Todoist API token |
| `config` | Manage configuration settings |
| `get` | Retrieve resources (tasks, projects, filters) |
| `add` | Create new resources (tasks) |
| `edit` | Modify existing resources |
| `complete` | Mark tasks as completed |
| `reopen` | Reopen completed tasks |
| `delete` | Remove resources |

### Command Usage Examples

#### init - Initialize Configuration

Initialize with your Todoist API token:

```bash
todorust init --api-token YOUR_API_TOKEN
```

Get your token from: https://todoist.com/app/settings/integrations

#### config - Manage Configuration

```bash
# View current configuration
todorust config show

# Update API token
todorust config set api-token NEW_TOKEN
```

#### get - Retrieve Resources

```bash
# Get tasks (default format: JSON)
todorust get tasks
todorust get tasks --format json

# Get tasks as checklist
todorust get tasks --format checklist

# Get tasks with filtering
todorust get tasks --filter "project:Work" --format structured

# Get projects
todorust get projects
todorust get projects --format checklist

# Get filters
todorust get filters
todorust get filters --format structured
```

#### add - Create Resources

```bash
# Create a new task
todorust add task --title "Buy milk"
todorust add task --title "Write report" --description "Draft Q1 summary"

# Create task with project and due date
todorust add task --title "Write report" --project-id "123" --due-date "2026-01-20" --priority 4

# Create task with labels
todorust add task --title "Urgent task" --labels "urgent,work"
```

#### edit - Modify Resources

```bash
# Edit a task
todorust edit task --task-id "456" --title "Updated title"
todorust edit task --task-id "456" --description "New description"
todorust edit task --task-id "456" --priority 3
```

#### complete - Complete Tasks

```bash
# Mark a task as completed
todorust complete task --task-id "456"
```

#### reopen - Reopen Tasks

```bash
# Reopen a completed task
todorust reopen task --task-id "456"
```

#### delete - Remove Resources

```bash
# Delete a task
todorust delete task --task-id "456"
```

## Output Formats

Todorust supports multiple output formats via the `--format` parameter:

**JSON (default):**
```bash
todorust get tasks
todorust get tasks --format json
```

**Markdown Checklist:**
```bash
todorust get tasks --format checklist
todorust get tasks --filter 'completed today' --format checklist
```

Output:
```markdown
- [x] Complete proposal (Work)
- [ ] Review docs (Work)
- [x] Buy groceries (Personal)
```

**Structured Markdown (by project):**
```bash
todorust get tasks --format structured
todorust get tasks --filter 'completed within "7 days"' --format structured
```

Output:
```markdown
## Personal

- [x] Buy groceries
- [ ] Pay bills (Priority: 2)

## Work

- [x] Complete proposal (Priority: 4)
- [ ] Review docs (Priority: 3)
```

## Legacy Commands (Backward Compatible)

The following legacy commands are still supported for backward compatibility:

| Legacy Command | New Command |
|----------------|-------------|
| `todorust tasks` | `todorust get tasks` |
| `todorust projects` | `todorust get projects` |
| `todorust filters` | `todorust get filters` |
| `todorust create` | `todorust add` |

## Development

```bash
# Run tests
cargo test

# Run with config
TODOIST_TOKEN=your_token cargo test -- --ignored
```
