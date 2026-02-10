---
name: plan-work
description: Use when receiving a new task or project to break it into trackable stories on the Stack kanban board before starting implementation
---

# Plan Work

## Overview

Break a task into stories on your Stack kanban board before writing code. This gives you a trackable plan and lets you work through stories methodically.

## Prerequisites

`stack` must be installed and on your PATH. If not available, tell the user to install it from the Stack repository and run `cargo install --path .`.

## Process

### 1. Check the current board state

```bash
stack board
```

Review what's already in progress or planned. Avoid duplicating existing work.

### 2. Create stories for the task

Break the task into small, independently completable stories. Each story should represent a meaningful unit of work.

```bash
stack story create --title "Short description of work" --priority medium --body "Detailed requirements and acceptance criteria"
```

**Guidelines:**
- Keep stories small enough to complete in one focused session
- Write clear acceptance criteria in the body
- Set priority based on dependency order (things that block others = higher priority)
- Stories are created in `todo` status by default

### 3. Optionally group under an epic

If the task is large, create an epic first:

```bash
stack epic create --title "Epic name"
```

Then assign stories to it:

```bash
stack story create --title "Story title" --epic <epic-id> --priority medium
```

### 4. Report the plan

After creating all stories, run `stack board` and present the plan to the user. Show what you created and the order you intend to work through them.

## Key Principles

- **Plan before coding** — All stories created before any implementation begins
- **Small stories** — Each story should be completable in one session
- **Clear acceptance criteria** — Write what "done" looks like in the body
- **Dependency order** — Higher priority for stories that unblock others
