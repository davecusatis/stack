---
name: view-board
description: Use when needing to check the current state of the Stack kanban board to orient yourself or report progress
---

# View Board

## Overview

Check the current board state to understand what's done, in progress, and remaining. Use at the start of a session, when resuming work, or when the user asks about progress.

## Prerequisites

`stack` must be installed and on your PATH. If not available, tell the user to install it from the Stack repository and run `cargo install --path .`.

## Process

### 1. Get the board snapshot

```bash
stack board
```

This returns the full board grouped by status columns: `todo`, `in_progress`, `in_review`, `done`.

### 2. Present a summary

Report to the user:
- How many stories are in each status
- What's currently in progress
- What's next in the todo column
- Overall progress (e.g., "3 of 7 stories complete")

### 3. Filter by epic (optional)

If working within a specific epic:

```bash
stack board --epic <epic-id>
```

## When to Use

- **Starting a session** — Orient yourself on where things stand
- **Resuming interrupted work** — Find what was in progress
- **User asks about progress** — Give a status update
- **After completing a story** — See what's next
