---
name: todorust
description: Manage Todoist tasks, projects, sections, filters, and labels via the `todorust` CLI. Use when a user asks to manage their Todoist data, including creating tasks, listing projects, completing tasks, or moving tasks between sections.
homepage: https://github.com/mimiqdev/todorust
metadata: {"gemini":{"emoji":"✅","requires":{"bins":["todorust"]}}}
---

# Todorust

Todoist CLI tool with optimized JSON output for AI agents.

## Setup

- Install: `cargo install --path .` (from project root)
- Configure: `todorust init --api-token YOUR_TOKEN`
- Format options: `--format json | checklist | structured`

## Commands

### Read Resources

```bash
# Get tasks (with optional filter)
todorust get tasks --filter "today"

# Get tasks in a specific project (filter by project name)
todorust get tasks --filter "Work"

# Get all projects
todorust get projects

# Get all sections (optionally for a project)
todorust get sections --project-id "12345678"

# Get all filters
todorust get filters

# Get all labels
todorust get labels
```

### Create Resources

```bash
# Basic task
todorust add task --title "Buy milk"

# Task with description, project, due date, and priority (1-4)
todorust add task --title "Review PR" --description "Check the sync logic" --project-id "222" --due-date "tomorrow" --priority 4

# Create a project
todorust add project --name "Side Project"

# Create a section in a project
todorust add section --name "Backlog" --project-id "222"
```

### Update/Move Resources

```bash
# Edit a task's title or priority
todorust edit task --task-id "123" --title "New Title" --priority 3

# Move a task to a different project/section
todorust move task --task-id "123" --project-id "456" --section-id "789"

# Complete a task
todorust complete task --task-id "123"

# Reopen a completed task
todorust reopen task --task-id "123"
```

### Delete Resources

```bash
# Delete a task
todorust delete task --task-id "123"

# Delete a project
todorust delete project --project-id "456"
```

## Filter Syntax (for `get tasks --filter`)

The `--filter` argument in `get tasks` currently performs a simple case-insensitive substring match on the task **content** or **project name**.

| Filter Type | Example |
|-------------|---------|
| By Content  | `todorust get tasks --filter "milk"` |
| By Project  | `todorust get tasks --filter "Work"` |

*Note: For complex Todoist filters (e.g., "due today & p1"), you may need to fetch all tasks and filter them yourself or use Todoist's native filter queries if supported by the backend.*

## Output Formats

- `json`: Full JSON output (default). Best for agents to parse IDs and details.
- `checklist`: Markdown checklist (`- [ ] task (Project)`).
- `structured`: Markdown grouped by project with headings.

## Examples for Agents

### 1. Finding a Task ID
If a user says "Complete my 'Buy milk' task", first search for it:
```bash
todorust get tasks --filter "Buy milk" --format json
```
Then use the `id` from the JSON to complete it:
```bash
todorust complete task --task-id "ID_FROM_JSON"
```

### 2. Organizing a Project
To see sections in a project:
```bash
todorust get sections --project-id "PROJECT_ID"
```
To move a task into a section:
```bash
todorust move task --task-id "TASK_ID" --project-id "PROJECT_ID" --section-id "SECTION_ID"
```
