---
name: complete-story
description: Use when finishing a story to mark it done on the Stack kanban board and pick up the next one
---

# Complete Story

## Overview

Mark the current story as done and check if there's more work to pick up.

## Prerequisites

`stack` must be installed and on your PATH. If not available, tell the user to install it from the Stack repository and run `cargo install --path .`.

## Process

### 1. Move story to done

```bash
stack story update <id> --status done
```

### 2. Check for remaining work

```bash
stack story list --status todo
```

If there are remaining stories in `todo`, inform the user and offer to start the next one using the `stack:start-story` skill.

If all stories are done, report completion:

```bash
stack board
```

Present the final board state showing all work completed.

## Key Principles

- **Mark done promptly** — Update status as soon as the story's acceptance criteria are met
- **Check the board** — Always look for remaining work after completing a story
- **Report progress** — Show the user what's done and what's left
