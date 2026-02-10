---
name: start-story
description: Use when picking up the next story from the Stack kanban board to begin implementation work
---

# Start Story

## Overview

Pick up the next story from the board, move it to in-progress, and load its context before starting work.

## Prerequisites

`stack` must be installed and on your PATH. If not available, tell the user to install it from the Stack repository and run `cargo install --path .`.

## Process

### 1. Find the next story

```bash
stack story list --status todo
```

Pick the highest priority story, or the next one in logical dependency order.

### 2. Move it to in-progress

```bash
stack story update <id> --status in-progress
```

### 3. Load the full context

```bash
stack story get <id>
```

Read the title, body, and any tasks. The body contains acceptance criteria and implementation notes.

### 4. Check for subtasks

```bash
stack task list <story-id>
```

If the story has tasks, use them as your implementation checklist. Toggle them as you complete each one:

```bash
stack task toggle <task-id>
```

### 5. Begin implementation

With the story context loaded, start working. Reference the acceptance criteria from the story body to know when you're done.

## Key Principles

- **One story at a time** — Only one story should be in-progress per session
- **Read before coding** — Always load the full story context first
- **Track progress** — Toggle subtasks as you complete them
