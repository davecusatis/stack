# Agent Instructions for Stack

## Project Overview

Stack is a personal terminal kanban board built in Rust (2024 edition). Single-binary TUI app using ratatui + crossterm for rendering and rusqlite (bundled) for persistence.

## Architecture

**Data flow:** Input (crossterm) → Action enum (input.rs) → handle_action (main.rs) → DB write (db.rs) + state mutation (app.rs) → refresh_board → UI render (ui/)

**Single state owner:** The `App` struct in `app.rs` owns all runtime state. No global mutable state, no Arc/Mutex. State is mutated in `handle_action()` and read in `ui::render()`.

**Immediate-mode rendering:** The entire screen is redrawn every frame. No retained widget state. Ratatui `Frame` is passed through render functions.

## File Layout

```
src/
├── main.rs        # Entry point, event loop, handle_action() dispatcher
├── app.rs         # App struct, Mode enum, navigation methods
├── db.rs          # Database struct, migrations, all CRUD (private conn)
├── models.rs      # Epic, Story, Status, Priority (no Task — removed)
├── actions.rs     # Action enum (flat, no logic)
├── input.rs       # Key → Action mapping per mode
└── ui/
    ├── mod.rs     # Render dispatcher + centered dialog helpers
    ├── board.rs   # Kanban columns
    ├── detail.rs  # Story detail with markdown body
    └── epic_list.rs  # Epic selector
```

## Gotchas

### Rust 2024 Edition
Requires `rustc 1.85+`. Uses let-chains (`if let ... && let ...`) extensively in `main.rs`. These won't compile on older toolchains.

### Bundled SQLite
`rusqlite` uses `features = ["bundled"]` — compiles SQLite from C source. First build is slow (~30s). No system SQLite dependency needed.

### Database Connection is Private
`Database.conn` is private. Tests construct `Database` directly because they live in the same module (`db.rs`). Don't try to access `conn` from outside `db.rs`.

### Legacy Tasks Table
The migration still creates a `tasks` table for backward compatibility with existing databases, but the app does not use it. Stories have markdown bodies (`description` field) instead of task checklists.

### list_selection is Context-Dependent
`app.list_selection` is reused across views:
- **Epic list:** index into epic list (0 = "All Epics", 1..n = specific epics)
- **Other views:** unused

When opening the epic list, `list_selection` is initialized to match the current `epic_filter`.

### InputConfirm Serves Double Duty
`Action::InputConfirm` handles both text input submission AND epic list selection. The handler checks `if app.mode == Mode::EpicList` first and returns early.

### Status Message Auto-Clears
`app.status_message` is set to `None` at the start of every `handle_action()` call. Messages only survive one action cycle.

### Column Selection Clamping
After any `refresh_board()`, `app.clamp_selections()` runs to prevent out-of-bounds card indices. Always call `clamp_selections()` after modifying `app.columns`.

## DB Enum Mapping

Status and Priority are stored as lowercase strings in SQLite:
- Status: `"todo"`, `"in_progress"`, `"in_review"`, `"done"`
- Priority: `"low"`, `"medium"`, `"high"`, `"critical"`

Conversion functions: `status_to_db`, `status_from_db`, `priority_to_db`, `priority_from_db` in `db.rs`.

## Testing

```bash
cargo test              # All 21 tests
cargo test db::         # DB tests only (8 tests)
cargo test models::     # Model tests (4 tests)
cargo test app::        # App state tests (4 tests)
cargo test input::      # Input handler tests (5 tests)
```

DB tests use `Connection::open_in_memory()` — no filesystem, no cleanup. The `test_db()` helper enables foreign keys and runs migrations.

UI modules have no tests (visual rendering). Verify visually with `cargo run`.

## Building

```bash
cargo build             # Dev build
cargo clippy -- -D warnings   # Lint (should be zero warnings)
cargo run               # Run the app
```

Data is stored at the platform-specific data directory:
- macOS: `~/Library/Application Support/stack/stack.db`
- Linux: `~/.local/share/stack/stack.db`

## Adding New Features

**New action:** Add variant to `Action` enum in `actions.rs` → add key mapping in `input.rs` → handle in `handle_action()` in `main.rs`.

**New DB operation:** Add method to `impl Database` in `db.rs` → add test using `test_db()` → call from `handle_action()`.

**New view mode:** Add variant to `Mode` enum in `app.rs` → add render function in `ui/` → add dispatch in `ui/mod.rs` → add input handler in `input.rs` → add mode transitions in `handle_action()`.
