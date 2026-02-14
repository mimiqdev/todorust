---
name: todoist-reports
description: Provides Todoist data formatted as Markdown (checklists, project summaries). Use for generating daily/weekly reports or fetching tasks to insert directly into Obsidian or other notes.
metadata: {"gemini":{"emoji":"📊","requires":{"bins":["todorust"]}}}
---

# Todoist Reports

Data retrieval skill for Todoist tasks and projects using todorust CLI. Provides formatted Markdown output ready for reports and note-taking.

## Quick Start

### Verify Installation

```bash
which todorust
todorust --version
```

### Basic Usage

**Get today's tasks as checklist:**
```bash
todorust get tasks --filter "today" --format checklist
```

**Get only task names and IDs (Token efficient):**
```bash
todorust get tasks --fields "id,content"
```

**Limit to most recent 5 tasks:**
```bash
todorust get tasks --limit 5
```

**Get tasks organized by project:**
```bash
todorust get tasks --format structured
```

## Output Formats for Obsidian

Todorust supports two Markdown-friendly formats:

### Checklist (`--format checklist`)

Simple Markdown checklist for daily notes:
```bash
todorust get tasks --filter "today" --format checklist
```

Output:
```markdown
- [x] Complete proposal (Work)
- [ ] Review docs (Work)
- [x] Buy groceries (Personal)
```

### Structured (`--format structured`)

Organized by project with priority info:
```bash
todorust get tasks --format structured
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

## Common Use Cases

### Daily Notes
Fetch today's tasks to insert into an Obsidian daily note:
```bash
todorust get tasks --filter "today" --format checklist
```

### Weekly Review
Generate a summary of all tasks for a weekly review:
```bash
todorust get tasks --format structured
```

### Project Dashboard
Fetch tasks for a specific project to display in a project-specific note:
```bash
todorust get tasks --filter "Work" --format checklist
```

## Integration Tips

- **Direct Insertion**: The output of `checklist` and `structured` formats is designed to be copy-pasted or piped directly into Markdown files.
- **Filtering**: Use the `--filter` flag to narrow down tasks by content or project name.
- **IDs**: If you need to perform actions (like completing a task from Obsidian), use the `json` format to get the `id`.

```bash
todorust get tasks --filter "Buy milk" --format json
```
