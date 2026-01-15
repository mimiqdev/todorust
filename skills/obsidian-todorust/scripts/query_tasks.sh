#!/bin/bash
# Helper script for common todorust queries
# Usage: ./query_tasks.sh <query_type> [options]
#
# Examples:
#   ./query_tasks.sh daily
#   ./query_tasks.sh weekly "Work"
#   ./query_tasks.sh custom "project:Work & completed within \"7 days of today\""

set -e

QUERY_TYPE="$1"
shift
EXTRA_ARGS="$@"

case "$QUERY_TYPE" in
  daily)
    # Tasks completed today
    todorust tasks --filter 'completed today'
    ;;
  weekly)
    # Tasks completed in last 7 days, optionally filtered by project
    if [ -n "$EXTRA_ARGS" ]; then
      PROJECT="$EXTRA_ARGS"
      todorust tasks --filter "project:$PROJECT & completed within \"7 days of today\""
    else
      todorust tasks --filter 'completed within "7 days of today"'
    fi
    ;;
  projects)
    # All projects
    todorust projects
    ;;
  active)
    # All active (incomplete) tasks
    todorust tasks
    ;;
  high-priority)
    # High priority tasks
    todorust tasks --filter 'priority:4'
    ;;
  custom)
    # Custom filter
    if [ -z "$EXTRA_ARGS" ]; then
      echo "Error: custom query requires filter argument" >&2
      echo "Usage: $0 custom \"project:Work & completed today\"" >&2
      exit 1
    fi
    todorust tasks --filter "$EXTRA_ARGS"
    ;;
  *)
    echo "Usage: $0 {daily|weekly|projects|active|high-priority|custom} [options]" >&2
    echo "" >&2
    echo "Examples:" >&2
    echo "  $0 daily                    # Today's completed tasks" >&2
    echo "  $0 weekly                   # All tasks completed this week" >&2
    echo "  $0 weekly Work              # Work tasks completed this week" >&2
    echo "  $0 projects                 # List all projects" >&2
    echo "  $0 active                   # All active tasks" >&2
    echo "  $0 high-priority            # High priority tasks" >&2
    echo "  $0 custom \"project:Work\"    # Custom filter" >&2
    exit 1
    ;;
esac
