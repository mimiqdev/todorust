---
name: obsidian-todorust
description: Todoist data retrieval for Obsidian using todorust CLI. Fetch tasks and projects in ready-to-use Markdown formats. Use when skills need to: (1) Get completed tasks for daily/weekly reports, (2) Query by project, date, or filters, (3) Generate checklists or structured reports, (4) Integrate Todoist data into Obsidian notes
---

# Obsidian Todoist Integration

Data retrieval skill for Todoist tasks and projects using todorust CLI. Provides formatted Markdown output ready for Obsidian notes. Use when other skills (daily notes, weekly reports) need Todoist data.

## Quick Start

### Verify Installation

```bash
which todorust
todorust --help
```

If not installed, see [installation guide](references/todorust-setup.md).

### Basic Usage

**Get today's completed tasks as checklist:**
```bash
todorust tasks --filter 'completed today' --format checklist
```

**Get this week's tasks organized by project:**
```bash
todorust tasks --filter 'completed within "7 days of today"' --format structured
```

**Get all tasks as JSON:**
```bash
todorust tasks
```

## Output Formats

Todorust supports three output formats via `--format` parameter:

### Checklist (`--format checklist`)

Simple Markdown checklist for daily notes:
```bash
todorust tasks --format checklist
```

Output:
```
- [x] Complete proposal (Work)
- [ ] Review docs (Work)
- [x] Buy groceries (Personal)
```

Rules:
- `- [x]` for completed tasks
- `- [ ]` for incomplete tasks
- Shows project name in parentheses
- Perfect for daily notes

### Structured (`--format structured`)

Organized by project with priority info:
```bash
todorust tasks --format structured
```

Output:
```
## Personal

- [x] Buy groceries
- [ ] Pay bills (Priority: 2)

## Work

- [x] Complete proposal (Priority: 4)
- [ ] Review docs (Priority: 3)
```

Rules:
- Grouped by project (## headings)
- Shows priority for tasks > 1
- Perfect for weekly reports

### JSON (default)

Full data structure for scripts:
```bash
todorust tasks
```

See [JSON schema reference](references/todorust-output.md).

## Common Filter Patterns

See [Todoist filter syntax](references/todoist-filters.md) for complete reference.

**Daily report:**
```bash
todorust tasks --filter 'completed today' --format checklist
```

**Weekly report:**
```bash
todorust tasks --filter 'completed within "7 days of today"' --format structured
```

**Project-specific:**
```bash
todorust tasks --filter 'project:Work & completed within "7 days"' --format structured
```

**High priority:**
```bash
todorust tasks --filter 'priority:4' --format checklist
```

## Integration Examples

### Daily Notes Skill

```bash
# Get today's completed tasks as ready-to-use checklist
todorust tasks --filter 'completed today' --format checklist
```

No parsing needed - direct output to note!

Output:
```
- [x] Complete proposal (Work)
- [x] Team meeting (Work)
- [x] Buy groceries (Personal)
```

### Weekly Review Skill

```bash
# Get this week's tasks organized by project
todorust tasks --filter 'completed within "7 days of today"' --format structured
```

Ready to paste into weekly review template!

Output:
```
## Personal

- [x] Buy groceries
- [ ] Pay bills (Priority: 2)

## Work

- [x] Complete proposal (Priority: 4)
- [x] Team meeting (Priority: 3)
- [ ] Review docs (Priority: 3)
```

### Custom Date Range

```bash
START_DATE="2026-01-10"
END_DATE="2026-01-16"
todorust tasks --filter "completed within \"$START_DATE to $END_DATE\"" --format structured
```

## Error Handling

**Config not found:**
```
Error: Configuration not found.
Run: todorust init --api-token YOUR_TOKEN
```

**API errors:**
- Check token is valid
- Check network connectivity
- Verify filter syntax

**Empty results:**
- Normal if no tasks match filter
- Returns empty output (no error)

## Performance

- API calls take 1-3 seconds typically
- Output is already formatted, no parsing needed
- todorust handles rate limiting internally

## Integration Notes

**For daily note skills:**
- Use `--format checklist` for task lists
- Direct output to notes, no transformation needed

**For weekly review skills:**
- Use `--format structured` for organized reports
- Output is ready for weekly templates

**For programmatic use:**
- Use `--format json` (default) for data processing
- See [JSON schema reference](references/todorust-output.md)
