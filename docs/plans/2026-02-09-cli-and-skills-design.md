# Stack — CLI & Skills Design

## Overview

Add subcommand-based CLI to the `stack` binary for programmatic access to all board operations. All output is JSON. Running `stack` with no args launches the TUI as today; any subcommand runs a CLI operation and exits.

Additionally, ship a set of workflow-oriented Claude Code skills that wrap the CLI, distributed as a shareable skill package via the marketplace.

## CLI Architecture

**Routing:** `stack` with no subcommand = TUI. Subcommand present = CLI operation.

**New dependency:** `clap` with `derive` feature for arg parsing.

### Subcommands

```
stack epic create --title "Auth System" [--description "..."] [--color "#ff0000"]
stack epic list
stack epic delete <id>

stack story create --title "Fix login bug" [--epic <id>] [--priority low|medium|high|critical] [--body "markdown..."]
stack story list [--epic <id>] [--status todo|in-progress|done|archived]
stack story get <id>
stack story update <id> [--title "..."] [--status ...] [--priority ...] [--body "..."]
stack story delete <id>

stack task create <story-id> --title "Write tests"
stack task list <story-id>
stack task toggle <task-id>
stack task delete <task-id>

stack board [--epic <id>]
```

## JSON Output Format

**Success** — returns a JSON object with a `result` key:

```json
{"result": {"id": 3, "title": "Fix login bug", "status": "todo"}}
```

**Lists:**

```json
{"result": [{"id": 1, ...}, {"id": 2, ...}]}
```

**Board snapshot** (`stack board`) — grouped by status:

```json
{"result": {"todo": [...], "in_progress": [...], "done": [...], "archived": [...]}}
```

**Errors** — stderr, non-zero exit code:

```json
{"error": "Story not found with id 42"}
```

Mutations (create/update/delete) return the affected entity so the caller has the ID and current state without a follow-up call.

## Code Changes

### New files

- **`src/cli.rs`** — Clap derive structs. Top-level `Cli` enum with optional subcommand. Subcommands: `Epic`, `Story`, `Task`, `Board` each with their own sub-subcommands.

- **`src/cli_handler.rs`** — Execution logic. Takes parsed CLI command, opens DB, runs the operation, serializes result as JSON to stdout. Errors serialize as JSON to stderr.

### Modified files

- **`src/main.rs`** — Parse args with clap first. If subcommand present, dispatch to `cli_handler`. Otherwise launch TUI.

- **`src/models.rs`** — Add `#[derive(Serialize)]` to Epic, Story, Task. Status and Priority get `Serialize` and `Deserialize` plus string-based parse support for CLI args.

- **`Cargo.toml`** — Add `clap`, `serde`, `serde_json` dependencies.

### Untouched

`db.rs`, `app.rs`, `input.rs`, `ui/` — the TUI is unchanged.

### New dependencies

```toml
clap = { version = "4", features = ["derive"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

## Skills Design

Four workflow-oriented skills distributed as a Claude Code skill package. The core flow: **plan work → execute stories → complete work**.

### `stack:plan-work`

Agent receives a task, reads the current board state for context, then creates a set of stories to accomplish the task. Outputs the created stories so the agent has their IDs for the session.

### `stack:start-story`

Agent picks up the next story. Moves it to `in-progress`, displays the story details (title, body, tasks), and sets the agent's context for working on it.

### `stack:complete-story`

Agent finishes a story. Moves it to `done`. Checks if there are remaining stories in `todo` and prompts the agent to pick up the next one.

### `stack:view-board`

Read-only. Agent checks the current board state to orient itself — what's done, what's in progress, what's left. Useful at the start of a session or when resuming work.

### Distribution

Skills live in a `skills/` directory at the repo root, structured for the Claude Code skill marketplace. Each skill is a markdown file with prompt and instructions, calling `stack` CLI commands via Bash.

Skills assume the user has `stack` installed and on their PATH. Each skill checks for this and gives a clear install message if not found.
