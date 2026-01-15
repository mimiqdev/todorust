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

### Get Tasks

```bash
# Get all tasks
todorust tasks

# Filter tasks
todorust tasks --filter "project:Work & due within 7 days"
```

### Get Projects

```bash
todorust projects
```

### Get Filters

```bash
todorust filters
```

### Create Task

```bash
# Basic task
todorust create --content "Buy milk"

# With project and due date
todorust create --content "Write report" --project-id "123" --due-date "2026-01-20" --priority 4
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
