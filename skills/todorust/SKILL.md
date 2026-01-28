---
name: todorust
description: Manage Todoist tasks via the `todorust` CLI (create/read/update/complete tasks, list projects/filters). Use when a user asks to add a task to Todoist, list tasks by project/date/priority, search tasks, or manage projects.
homepage: https://github.com/mimiqdev/todorust
metadata: {"clawdbot":{"emoji":"✅","requires":{"bins":["todorust"]}}}
---

# Todorust

Todoist CLI tool with optimized JSON output.

## Setup

- Install: `cargo install --git https://github.com/mimiqdev/todorust`
- Configure: `todorust init` (sets API token)
- Format options: `--format json | checklist | structured`

## Commands

### Read Tasks

```bash
# All tasks (with optional filter)
todorust tasks "project:Work & priority:4"

# Today's tasks
todorust tasks "due today"

# Upcoming tasks
todorust tasks "due within \"7 days of today\""

# Search tasks
todorust tasks "search query"

# Completed recently
todorust tasks "completed within \"7 days of today\""
```

### Projects & Filters

```bash
# List all projects
todorust projects

# List custom filters
todorust filters
```

### Create Task

```bash
# Basic task
todorust create "Buy milk"

# With description
todorust create "Buy milk" --description "2% milk + bananas"

# With due date
todorust create "Call dentist" --due "2026-01-15"

# Into project
todorust create "Finish report" --project "Work"

# With priority (1=none, 2=low, 3=normal, 4=high)
todorust create "Urgent task" --priority 4

# With labels
todorust create "Meeting" --labels "work,meeting"

# Checklist format
todorust create "Shopping" --format checklist
```

### Complete/Reopen

```bash
# Complete task (by ID)
todorust complete "12345"

# Reopen task
todorust reopen "12345"
```

## Filter Syntax

| Filter | Example |
|--------|---------|
| By Project | `project:Work` |
| By Date | `due today`, `due tomorrow`, `due within "7 days of today"` |
| By Priority | `priority:4` (high), `priority:3` (normal), `priority:2` (low), `priority:1` (none) |
| Combined | `project:Work & priority:4 & !completed` |

## Output Formats

- `json`: Full JSON output (default for scripting)
- `checklist`: Human-readable checklist
- `structured`: Compact table format

## Examples

### List today's tasks
```bash
todorust tasks "due today" --format checklist
```

### Add high-priority task to Work project
```bash
todorust create "Review PR #86" --project "Work" --priority 4 --labels "review,github"
```

### Find and complete a task
```bash
todorust tasks "Buy milk" --format json
# Find the ID, then:
todorust complete "12345"
```

### Weekly review
```bash
todorust tasks "completed within \"7 days of today\"" --format checklist
```
