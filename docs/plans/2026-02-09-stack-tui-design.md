# Stack — Terminal Task Tracker Design

## Overview

Stack is a personal terminal UI task tracker built in Rust. It presents a kanban board for managing work organized into epics, stories, and tasks.

## Data Model

Three-level hierarchy: **Epic > Story > Task**.

**Epic** — A large body of work. Has a title, description, and color label. An epic has no kanban status — its progress is implied by its stories' statuses.

**Story** — The primary unit of work, displayed as a card on the kanban board. Each story belongs to one epic (or none). Fields:
- Title, description
- Status: `ToDo`, `InProgress`, `InReview`, `Done`
- Priority: `Low`, `Medium`, `High`, `Critical`
- Created/updated timestamps
- Ordered list of tasks

**Task** — A checklist item within a story. Fields: title, done boolean, sort order.

**Storage** — SQLite via `rusqlite` (bundled). Three tables (`epics`, `stories`, `tasks`) with foreign keys. Database path: `~/.local/share/stack/stack.db`. Column definitions are an enum in code, not stored in the database.

## UI Layout

```
┌─────────────────────────────────────────────────────┐
│  Stack  │  Epic: Authentication System        [?]   │
├────────────┬────────────┬────────────┬──────────────┤
│  To Do     │ In Progress│ In Review  │    Done      │
│            │            │            │              │
│ ┌────────┐ │ ┌────────┐ │            │ ┌──────────┐ │
│ │Story A │ │ │Story C │ │            │ │ Story B  │ │
│ │ P:High │ │ │ 2/5    │ │            │ │ 3/3  ✓   │ │
│ └────────┘ │ └────────┘ │            │ └──────────┘ │
│ ┌────────┐ │            │            │              │
│ │Story D │ │            │            │              │
│ │ P:Med  │ │            │            │              │
│ └────────┘ │            │            │              │
├────────────┴────────────┴────────────┴──────────────┤
│  j/k: nav  h/l: column  Enter: open  n: new  ?: help│
└─────────────────────────────────────────────────────┘
```

- **Header** — App name, current epic filter, help hint.
- **Board** — Four equal-width columns. Story cards show title, priority, and task completion count. Selected card is highlighted.
- **Footer** — Context-sensitive keybinding hints.

## Views

- **Board view** (default) — Kanban columns with story cards.
- **Story detail view** — Full description, task checklist, metadata. Opened with Enter.
- **Epic list view** — List of epics for filtering/switching.

## Application Architecture

```
src/
├── main.rs          # Entry point, terminal setup, event loop
├── app.rs           # App state, mode transitions, update logic
├── db.rs            # SQLite connection, migrations, CRUD
├── models.rs        # Epic, Story, Task structs + Status/Priority enums
├── ui/
│   ├── mod.rs       # Render dispatch based on current mode
│   ├── board.rs     # Kanban board rendering
│   ├── detail.rs    # Story detail view
│   └── epic_list.rs # Epic list/selector
├── input.rs         # Key event handling, maps keys to actions per mode
└── actions.rs       # Business logic: move story, toggle task, create/delete
```

**App state** — A single `App` struct owns all runtime state: current mode, selected column/card indices, epic filter, and cached story data from the DB. No global mutable state.

**Event loop:**
1. `crossterm::event::poll()` for input
2. Map key to `Action` enum variant
3. `App::update(action)` mutates state, writes to DB
4. `ui::render(&app, &mut frame)` draws the current view

**Database** — Reads cached in App state, refreshed after mutations. Writes go directly to SQLite. `db::migrate()` creates tables on first run.

## Dependencies

- `ratatui` + `crossterm` — TUI rendering and terminal backend
- `rusqlite` (bundled feature) — SQLite with no system dependency
- `dirs` — XDG-compliant path resolution

## Keybindings

### Board view
| Key | Action |
|-----|--------|
| `h/l` | Move focus between columns |
| `j/k` | Move focus between cards in a column |
| `H/L` | Move selected story to previous/next column |
| `Enter` | Open story detail view |
| `n` | New story in current column |
| `e` | Open epic list view |
| `d` | Delete story (with confirmation) |
| `q` / `Ctrl+C` | Quit |

### Story detail view
| Key | Action |
|-----|--------|
| `j/k` | Navigate task checklist |
| `Space` | Toggle task done/not-done |
| `n` | Add new task |
| `e` | Edit story title/description |
| `d` | Delete task under cursor |
| `Esc` | Back to board |

### Text input
- `Enter` — Confirm input
- `Esc` — Cancel input

Descriptions are single-line for v1.

## Error Handling

- SQLite/terminal errors display as transient messages in the footer bar.
- All DB operations return `Result` — no panics on DB errors.
- Startup failure (can't create DB) prints to stderr and exits non-zero before entering TUI.

## Startup Flow

1. Resolve `~/.local/share/stack/`, create if missing
2. Open/create SQLite DB, run migrations
3. Enter alternate screen, enable raw mode
4. Load initial data, start event loop
5. On quit: restore terminal, exit cleanly
