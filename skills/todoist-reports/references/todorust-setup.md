# Todorust Installation and Setup

Guide to install and configure todorust CLI tool.

## Prerequisites

- Rust toolchain (for building from source)
- Todoist account
- Todoist API token

## Installation

### From Source

If you're in the todorust project directory:

```bash
cargo build --release
```

The binary will be at:
```
./target/release/todorust
```

Add to PATH (replace `/path/to/todorust` with your actual path):
```bash
export PATH="$PATH:/path/to/todorust/target/release"
```

### Via Cargo Install (Recommended)

```bash
cargo install --path .
```

## Configuration

### Get Todoist API Token

1. Go to https://todoist.com/app/settings/integrations
2. Find "API token" section
3. Copy the token.

### Initialize Todorust

```bash
todorust init --api-token YOUR_API_TOKEN_HERE
```

This creates a configuration file at `~/.config/todorust/config.toml` (or equivalent on your OS).

## Verify Installation

### Check Version

```bash
todorust --version
```

### Test Commands

```bash
# Get projects
todorust get projects

# Get all tasks
todorust get tasks

# Search for a task
todorust get tasks --filter "milk"
```

## Troubleshooting

### Config Not Found

**Problem:** `Error: Configuration not found.`

**Solution:**
```bash
todorust init --api-token YOUR_TOKEN
```

### API Errors

**Problem:** `Error: HTTP 401 - Unauthorized`

**Solution:**
- Check token is correct
- Re-run: `todorust init --api-token NEW_TOKEN`

## For Obsidian Integration

When using with Obsidian + AI skills:

1. **Ensure todorust is in PATH** so AI can invoke it.
2. **Verify config exists** so no interactive setup is needed.
3. **Use JSON format** if you need to parse IDs for actions.

**Recommended test sequence:**
```bash
# 1. Basic check
todorust get projects

# 2. Get tasks
todorust get tasks --format checklist
```
