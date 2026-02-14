# AI Agent Optimization Plan

This document outlines enhancements to `todorust` specifically designed to improve the experience for AI Agents (LLMs) and automated workflows.

## Goals

### 1. Batch Operations (Sync API Power)
Fully leverage the Sync API by allowing multiple commands in a single request.
- **Action**: Add a `batch` command that accepts a JSON array of operations.
- **Benefit**: Reduces network round-trips and tool call overhead for the AI.

### 2. Enhanced Filter Fault-tolerance
Improve how the CLI handles filters to be more resilient to LLM-generated queries.
- **Action**: Better internal documentation/hints for the AI and potentially pre-processing common natural language patterns.
- **Benefit**: Increases the success rate of task retrieval.

### 3. Field Selection (`--fields`)
Allow the user/AI to specify which fields to return in the JSON output.
- **Action**: Add a `--fields` (or `--select`) parameter to `get` commands.
- **Benefit**: Significantly reduces token consumption when dealing with large task lists.

### 4. Robust ID Mapping
Ensure that when tasks are created via the Sync API, the mapping from `temp_id` to `real_id` is clearly returned and easy for the AI to follow.
- **Action**: Standardize the response format for creation and batch commands to include the ID mapping clearly.
- **Benefit**: Enables the AI to perform follow-up actions on newly created items within the same context.

---
*Created: 2026-02-14*
