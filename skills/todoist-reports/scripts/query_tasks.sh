#!/bin/bash
# Helper script for common todorust queries
# Usage: ./query_tasks.sh <query_type> [options]
#
# Examples:
#   ./query_tasks.sh daily
#   ./query_tasks.sh weekly
#   ./query_tasks.sh projects

set -e

QUERY_TYPE="$1"
shift
EXTRA_ARGS="$@"

case "$QUERY_TYPE" in
  daily)
    # Tasks with "today" in title or project (Simple filter)
    todorust get tasks --filter "today"
    ;;
  weekly)
    # All tasks (AI can filter for weekly reports)
    todorust get tasks
    ;;
  projects)
    # All projects
    todorust get projects
    ;;
  active)
    # All active (incomplete) tasks
    todorust get tasks
    ;;
  custom)
    # Custom simple filter
    if [ -z "$EXTRA_ARGS" ]; then
      echo "Error: custom query requires filter argument" >&2
      echo "Usage: $0 custom \"search string\"" >&2
      exit 1
    fi
    todorust get tasks --filter "$EXTRA_ARGS"
    ;;
  *)
    echo "Usage: $0 {daily|weekly|projects|active|custom} [options]" >&2
    echo "" >&2
    echo "Examples:" >&2
    echo "  $0 daily                    # Tasks matching 'today'" >&2
    echo "  $0 weekly                   # All tasks" >&2
    echo "  $0 projects                 # List all projects" >&2
    echo "  $0 active                   # All active tasks" >&2
    echo "  $0 custom \"Work\"            # Tasks matching 'Work'" >&2
    exit 1
    ;;
esac
