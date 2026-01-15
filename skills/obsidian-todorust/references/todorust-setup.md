# Todorust Installation and Setup

Guide to install and configure todorust CLI tool.

## Prerequisites

- Rust toolchain (for building from source)
- Todoist account
- Todoist API token

## Installation

### From Source (Current Project)

If you're in the todorust project directory:

```bash
cd /home/tonyliu/Developer/projects/todoirust
cargo build --release
```

The binary will be at:
```
./target/release/todorust
```

Add to PATH:
```bash
# Temporary
export PATH="$PATH:/home/tonyliu/Developer/projects/todoirust/target/release"

# Permanent (add to ~/.bashrc or ~/.zshrc)
echo 'export PATH="$PATH:/home/tonyliu/Developer/projects/todoirust/target/release"' >> ~/.bashrc
source ~/.bashrc
```

### Via Cargo Install (When Published)

When todorust is published to crates.io:

```bash
cargo install todorust
```

## Configuration

### Get Todoist API Token

1. Go to https://todoist.com/app/settings/integrations
2. Find "API token" section
3. Click "Reset token" if needed
4. Copy the token (starts with length ~40 characters)

### Initialize Todorust

```bash
todorust init --api-token YOUR_API_TOKEN_HERE
```

This creates:
```
~/.config/todoirust/config.toml
```

With content:
```toml
api_token = "your_token_here"
```

## Verify Installation

### Check Version

```bash
todorust --version
```

Should show: `todorust 0.1.0`

### Test Commands

```bash
# Get projects
todorust projects

# Get all tasks
todorust tasks

# Get completed tasks today
todorust tasks --filter 'completed today'
```

Expected: JSON output with tasks/projects.

## Troubleshooting

### Command Not Found

**Problem:** `todorust: command not found`

**Solution:**
```bash
# If building from source
export PATH="$PATH:/home/tonyliu/Developer/projects/todoirust/target/release"

# Or add to ~/.bashrc permanently
echo 'export PATH="$PATH:/home/tonyliu/Developer/projects/todoirust/target/release"' >> ~/.bashrc
```

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
- Regenerate token in Todoist settings
- Re-run: `todorust init --api-token NEW_TOKEN`

### Empty Results

**Problem:** Returns `[]` but tasks exist

**Possible causes:**
1. Filter too restrictive → Try simpler filter
2. Token permissions → Verify token has access
3. API rate limit → Wait a few seconds
4. Sync delay → Tasks may take time to sync

## For Obsidian Integration

When using with Obsidian + AI skills:

1. **Ensure todorust is in PATH** so AI can invoke it
2. **Verify config exists** so no interactive setup needed
3. **Test with your common filters** before integrating

**Recommended test sequence:**
```bash
# 1. Basic check
todorust projects

# 2. Daily report query
todorust tasks --filter 'completed today'

# 3. Weekly report query
todorust tasks --filter 'completed within "7 days of today"'

# 4. Project-specific query
todorust tasks --filter 'project:Work & completed within "7 days of today"'
```

All should return JSON arrays (may be empty `[]` if no matching tasks).

## Updating

### From Source

```bash
cd /home/tonyliu/Developer/projects/todoirust
git pull
cargo build --release
```

Binary will be updated at `./target/release/todorust`

### Reinstall (if published)

```bash
cargo install todorust --force
```

## Uninstalling

### Remove Binary

```bash
# If installed via cargo
cargo uninstall todorust

# If using from source
rm /home/tonyliu/Developer/projects/todoirust/target/release/todorust
```

### Remove Config

```bash
rm -rf ~/.config/todoirust
```
