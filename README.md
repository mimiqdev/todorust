# Todorust CLI

A Rust CLI tool for Todoist API integration with simplified JSON output for AI tools and automation workflows.

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

## Configuration

Initialize with your Todoist API token:

```bash
todorust init --api-token YOUR_API_TOKEN
```

Get your token from: https://todoist.com/app/settings/integrations

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
- [ ] Pay bills (Priority: 2)

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

### Create Task

```bash
# Basic task (new API fields)
todorust create --title "Buy milk"
todorust create --title "Write report" --description "Draft Q1 summary"

# Backward compatible content flag
todorust create --content "Buy milk"

# With project and due date
todorust create --title "Write report" --project-id "123" --due-date "2026-01-20" --priority 4

# With labels
todorust create --title "Urgent task" --labels "urgent,work"
```

### Complete/Reopen Task

```bash
todorust complete --task-id "456"
todorust reopen --task-id "456"
```

## Development

```bash
# Run tests
cargo test

# Run with config
TODOIST_TOKEN=your_token cargo test -- --ignored
```
