# Todorust CLI

![CI](https://github.com/mimiqdev/todorust/actions/workflows/ci.yml/badge.svg)
![Coverage](https://codecov.io/gh/mimiqdev/todorust/branch/main/graph/badge.svg)

A Rust CLI tool for Todoist Sync API integration with simplified JSON output for AI tools and automation workflows.

## Features

- **Local Caching**: Automatic incremental sync with local cache to reduce API calls.
- **Batch Operations**: Execute multiple commands in a single Sync request.
- **AI-Optimized Output**: JSON responses for all actions, with field selection (`--fields`) to save tokens.
- **Advanced Filtering**: Support for priority and status keywords in task queries.
- **Smart Formatting**: Markdown checklists and structured project views.
- **Shell Completion**: Native support for bash, zsh, and fish.

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
| `get` | Retrieve resources (tasks, projects, filters, sections, labels) |
| `add` | Create new resources (tasks, projects, sections, filters, labels) |
| `edit` | Modify existing resources (tasks, sections, filters, labels) |
| `move` | Move tasks between sections/projects |
| `complete` | Mark tasks as completed |
| `reopen` | Reopen completed tasks |
| `delete` | Remove resources |
| `reorder` | Reorder sections |
| `sync` | Sync data with Todoist (uses local cache) |
| `cache` | Manage local cache (status, clear) |

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
todorust config get

# Update API token
todorust config set --api-token NEW_TOKEN
```

#### get - Retrieve Resources

```bash
# Get tasks (default format: JSON)
todorust get tasks
todorust get tasks --format json

# Get tasks with field selection (AI-optimized, saves tokens)
todorust get tasks --fields "id,content,priority" --limit 10

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

# Get labels
todorust get labels
todorust get labels --format structured

# Get sections
todorust get sections
todorust get sections --project-id "123"
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

# Create a new project
todorust add project --name "New Project"

# Create a new section
todorust add section --name "New Section" --project-id "123"

# Create a new filter
todorust add filter --name "My Filter" --query "query string"

# Create a new label
todorust add label --name "new-label"
```

#### edit - Modify Resources

```bash
# Edit a task
todorust edit task --task-id "456" --title "Updated title"
todorust edit task --task-id "456" --description "New description"
todorust edit task --task-id "456" --priority 3

# Edit a section
todorust edit section --section-id "123" --name "New section name"

# Edit a filter
todorust edit filter --filter-id "123" --name "Updated filter" --query "query string"

# Edit a label
todorust edit label --label-id "123" --name "Updated label name"
```

#### move - Move Tasks

```bash
# Move a task to a different section
todorust move task --task-id "456" --section-id "789"

# Move a task to a different project
todorust move task --task-id "456" --project-id "101"
```

#### reorder - Reorder Sections

```bash
# Reorder sections within a project
todorust reorder sections --section-ids "456,789,101"
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

#### batch - Execute Multiple Commands

Batch operations allow you to execute multiple commands in a single Sync API request. This is highly efficient for AI agents and automation.

```bash
# Execute multiple commands from a JSON array
todorust batch '[
  {"type": "item_add", "args": {"content": "First task"}},
  {"type": "item_add", "args": {"content": "Second task", "priority": 4}},
  {"type": "item_complete", "args": {"id": "123456"}},
  {"type": "project_add", "args": {"name": "New Batch Project"}}
]'
```

The command returns a JSON object containing the `sync_status` for each command (by UUID) and any `temp_id_mapping` for newly created resources.

### AI-Agent Optimization

Todorust is designed specifically to be used by AI agents (like LLMs):

1.  **JSON by Default**: All commands output structured JSON unless specified otherwise.
2.  **Field Selection**: Use `--fields "id,content,due"` to reduce the context window size and save tokens.
3.  **Result Limiting**: Use `--limit 5` to keep responses concise.
4.  **Batching**: Combine multiple mutations into a single `batch` call to reduce latency and API overhead.

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

## Local Caching

Todorust uses local caching to reduce API calls and improve performance:

```bash
# Sync data with Todoist (uses incremental sync by default)
todorust sync

# Force full sync
todorust sync --force

# Check cache status
todorust cache status

# Clear local cache
todorust cache clear
```

### How It Works

- **First sync**: Full sync downloads all data and stores it locally
- **Subsequent syncs**: Incremental sync only downloads changes (uses `sync_token`)
- **Cache expiry**: Default 5 minutes, configurable via `TODORUST_CACHE_TTL` env var
- **Hybrid mode**: Get commands use cache when valid, commands trigger sync

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `TODORUST_CACHE_TTL` | Cache expiry in seconds | 300 (5 min) |
| `TODORUST_SYNC_URL` | Custom sync API URL | api.todoist.com |

## Agent Skills

Todorust provides built-in skills for AI agents:

- **todorust**: Core management skill (create, edit, move, delete).
- **todoist-reports**: Data retrieval and formatting (checklists, summaries for note-taking).

See the `skills/` directory for details.

## Development

```bash
# Run tests
cargo test

# Setup Git hooks (auto-format and strict clippy)
git config core.hooksPath scripts/hooks
```
