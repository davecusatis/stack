# Stack TUI Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build a terminal kanban board app with epic/story/task hierarchy, SQLite storage, and vim-style navigation.

**Architecture:** Single-binary Rust TUI using ratatui + crossterm for rendering, rusqlite for persistence. App struct owns all state; event loop polls input, dispatches actions, re-renders. DB reads are cached in memory and refreshed after writes.

**Tech Stack:** Rust 2024 edition, ratatui 0.30, crossterm 0.29, rusqlite 0.38 (bundled), dirs 6.0

**Design doc:** `docs/plans/2026-02-09-stack-tui-design.md`

---

### Task 1: Add Dependencies

**Files:**
- Modify: `Cargo.toml`

**Step 1: Add all crate dependencies to Cargo.toml**

```toml
[dependencies]
ratatui = "0.30"
crossterm = "0.29"
rusqlite = { version = "0.38", features = ["bundled"] }
dirs = "6.0"
```

**Step 2: Verify it compiles**

Run: `cargo build`
Expected: Compiles successfully (will download and build dependencies, may take a minute for SQLite bundled build)

**Step 3: Commit**

```bash
git add Cargo.toml Cargo.lock
git commit -m "feat: add dependencies (ratatui, crossterm, rusqlite, dirs)"
```

---

### Task 2: Define Data Models

**Files:**
- Create: `src/models.rs`
- Modify: `src/main.rs` (add `mod models;`)

**Step 1: Write tests for model types**

In `src/models.rs`, add tests verifying enum conversions and struct construction:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_display_and_variants() {
        assert_eq!(Status::ToDo.as_str(), "To Do");
        assert_eq!(Status::InProgress.as_str(), "In Progress");
        assert_eq!(Status::InReview.as_str(), "In Review");
        assert_eq!(Status::Done.as_str(), "Done");
    }

    #[test]
    fn status_all_returns_four_columns() {
        assert_eq!(Status::all().len(), 4);
    }

    #[test]
    fn priority_ordering() {
        assert!(Priority::Critical > Priority::High);
        assert!(Priority::High > Priority::Medium);
        assert!(Priority::Medium > Priority::Low);
    }

    #[test]
    fn priority_display() {
        assert_eq!(Priority::Low.as_str(), "Low");
        assert_eq!(Priority::Critical.as_str(), "Critical");
    }

    #[test]
    fn task_completion() {
        let task = Task {
            id: 1,
            story_id: 1,
            title: "Write tests".to_string(),
            done: false,
            sort_order: 0,
        };
        assert!(!task.done);
    }
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test --lib models`
Expected: FAIL — module doesn't exist yet

**Step 3: Implement models**

```rust
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    ToDo,
    InProgress,
    InReview,
    Done,
}

impl Status {
    pub fn as_str(&self) -> &'static str {
        match self {
            Status::ToDo => "To Do",
            Status::InProgress => "In Progress",
            Status::InReview => "In Review",
            Status::Done => "Done",
        }
    }

    pub fn all() -> &'static [Status] {
        &[Status::ToDo, Status::InProgress, Status::InReview, Status::Done]
    }

    pub fn next(&self) -> Option<Status> {
        match self {
            Status::ToDo => Some(Status::InProgress),
            Status::InProgress => Some(Status::InReview),
            Status::InReview => Some(Status::Done),
            Status::Done => None,
        }
    }

    pub fn prev(&self) -> Option<Status> {
        match self {
            Status::ToDo => None,
            Status::InProgress => Some(Status::ToDo),
            Status::InReview => Some(Status::InProgress),
            Status::Done => Some(Status::InReview),
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

impl Priority {
    pub fn as_str(&self) -> &'static str {
        match self {
            Priority::Low => "Low",
            Priority::Medium => "Medium",
            Priority::High => "High",
            Priority::Critical => "Critical",
        }
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone)]
pub struct Epic {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub color: String,
}

#[derive(Debug, Clone)]
pub struct Story {
    pub id: i64,
    pub epic_id: Option<i64>,
    pub title: String,
    pub description: String,
    pub status: Status,
    pub priority: Priority,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone)]
pub struct Task {
    pub id: i64,
    pub story_id: i64,
    pub title: String,
    pub done: bool,
    pub sort_order: i32,
}
```

Add `mod models;` to `src/main.rs`.

**Step 4: Run tests to verify they pass**

Run: `cargo test --lib models`
Expected: All 5 tests PASS

**Step 5: Commit**

```bash
git add src/models.rs src/main.rs
git commit -m "feat: add data models (Epic, Story, Task, Status, Priority)"
```

---

### Task 3: Database Layer — Schema & Migrations

**Files:**
- Create: `src/db.rs`
- Modify: `src/main.rs` (add `mod db;`)

**Step 1: Write test for migration**

In `src/db.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    fn test_db() -> Database {
        let conn = Connection::open_in_memory().unwrap();
        let mut db = Database { conn };
        db.migrate().unwrap();
        db
    }

    #[test]
    fn migrate_creates_tables() {
        let db = test_db();
        // Verify tables exist by querying sqlite_master
        let count: i32 = db.conn.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name IN ('epics', 'stories', 'tasks')",
            [],
            |row| row.get(0),
        ).unwrap();
        assert_eq!(count, 3);
    }

    #[test]
    fn migrate_is_idempotent() {
        let mut db = test_db();
        // Running migrate again should not error
        db.migrate().unwrap();
    }
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test --lib db`
Expected: FAIL — module doesn't exist

**Step 3: Implement Database struct and migrate**

```rust
use rusqlite::{Connection, Result};

pub struct Database {
    pub conn: Connection,
}

impl Database {
    pub fn open(path: &std::path::Path) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;
        Ok(Database { conn })
    }

    pub fn migrate(&mut self) -> Result<()> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS epics (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                description TEXT NOT NULL DEFAULT '',
                color TEXT NOT NULL DEFAULT 'white'
            );

            CREATE TABLE IF NOT EXISTS stories (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                epic_id INTEGER REFERENCES epics(id) ON DELETE SET NULL,
                title TEXT NOT NULL,
                description TEXT NOT NULL DEFAULT '',
                status TEXT NOT NULL DEFAULT 'todo',
                priority TEXT NOT NULL DEFAULT 'medium',
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                updated_at TEXT NOT NULL DEFAULT (datetime('now'))
            );

            CREATE TABLE IF NOT EXISTS tasks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                story_id INTEGER NOT NULL REFERENCES stories(id) ON DELETE CASCADE,
                title TEXT NOT NULL,
                done INTEGER NOT NULL DEFAULT 0,
                sort_order INTEGER NOT NULL DEFAULT 0
            );"
        )?;
        Ok(())
    }
}
```

Add `mod db;` to `src/main.rs`.

**Step 4: Run tests to verify they pass**

Run: `cargo test --lib db`
Expected: All 2 tests PASS

**Step 5: Commit**

```bash
git add src/db.rs src/main.rs
git commit -m "feat: add database layer with schema migrations"
```

---

### Task 4: Database Layer — Epic CRUD

**Files:**
- Modify: `src/db.rs`

**Step 1: Write tests for epic CRUD**

Add to the `tests` module in `src/db.rs`:

```rust
#[test]
fn create_and_list_epics() {
    let db = test_db();
    let id = db.create_epic("Auth System", "Build login/signup", "blue").unwrap();
    assert!(id > 0);

    let epics = db.list_epics().unwrap();
    assert_eq!(epics.len(), 1);
    assert_eq!(epics[0].title, "Auth System");
    assert_eq!(epics[0].color, "blue");
}

#[test]
fn delete_epic() {
    let db = test_db();
    let id = db.create_epic("Temp", "", "white").unwrap();
    db.delete_epic(id).unwrap();
    let epics = db.list_epics().unwrap();
    assert_eq!(epics.len(), 0);
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test --lib db`
Expected: FAIL — methods don't exist

**Step 3: Implement epic CRUD**

Add to `impl Database`:

```rust
use crate::models::{Epic, Story, Task, Status, Priority};

pub fn create_epic(&self, title: &str, description: &str, color: &str) -> Result<i64> {
    self.conn.execute(
        "INSERT INTO epics (title, description, color) VALUES (?1, ?2, ?3)",
        rusqlite::params![title, description, color],
    )?;
    Ok(self.conn.last_insert_rowid())
}

pub fn list_epics(&self) -> Result<Vec<Epic>> {
    let mut stmt = self.conn.prepare("SELECT id, title, description, color FROM epics ORDER BY id")?;
    let epics = stmt.query_map([], |row| {
        Ok(Epic {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
            color: row.get(3)?,
        })
    })?.collect::<Result<Vec<_>>>()?;
    Ok(epics)
}

pub fn delete_epic(&self, id: i64) -> Result<()> {
    self.conn.execute("DELETE FROM epics WHERE id = ?1", [id])?;
    Ok(())
}
```

**Step 4: Run tests to verify they pass**

Run: `cargo test --lib db`
Expected: All 4 tests PASS

**Step 5: Commit**

```bash
git add src/db.rs
git commit -m "feat: add epic CRUD operations"
```

---

### Task 5: Database Layer — Story CRUD

**Files:**
- Modify: `src/db.rs`

Helper functions needed for converting between DB strings and enums. Add to `src/db.rs` (outside impl):

```rust
fn status_to_db(s: &Status) -> &'static str {
    match s {
        Status::ToDo => "todo",
        Status::InProgress => "in_progress",
        Status::InReview => "in_review",
        Status::Done => "done",
    }
}

fn status_from_db(s: &str) -> Status {
    match s {
        "in_progress" => Status::InProgress,
        "in_review" => Status::InReview,
        "done" => Status::Done,
        _ => Status::ToDo,
    }
}

fn priority_to_db(p: &Priority) -> &'static str {
    match p {
        Priority::Low => "low",
        Priority::Medium => "medium",
        Priority::High => "high",
        Priority::Critical => "critical",
    }
}

fn priority_from_db(s: &str) -> Priority {
    match s {
        "low" => Priority::Low,
        "high" => Priority::High,
        "critical" => Priority::Critical,
        _ => Priority::Medium,
    }
}
```

**Step 1: Write tests for story CRUD**

Add to the `tests` module:

```rust
#[test]
fn create_and_list_stories_by_status() {
    let db = test_db();
    let eid = db.create_epic("E1", "", "white").unwrap();
    db.create_story("Story A", "", Some(eid), Status::ToDo, Priority::High).unwrap();
    db.create_story("Story B", "", Some(eid), Status::InProgress, Priority::Low).unwrap();
    db.create_story("Story C", "", None, Status::ToDo, Priority::Medium).unwrap();

    let todo = db.list_stories_by_status(Status::ToDo, None).unwrap();
    assert_eq!(todo.len(), 2);

    let filtered = db.list_stories_by_status(Status::ToDo, Some(eid)).unwrap();
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].title, "Story A");
}

#[test]
fn update_story_status() {
    let db = test_db();
    let sid = db.create_story("S1", "", None, Status::ToDo, Priority::Medium).unwrap();
    db.update_story_status(sid, Status::InProgress).unwrap();

    let stories = db.list_stories_by_status(Status::InProgress, None).unwrap();
    assert_eq!(stories.len(), 1);
    assert_eq!(stories[0].id, sid);
}

#[test]
fn delete_story() {
    let db = test_db();
    let sid = db.create_story("S1", "", None, Status::ToDo, Priority::Medium).unwrap();
    db.delete_story(sid).unwrap();
    let stories = db.list_stories_by_status(Status::ToDo, None).unwrap();
    assert_eq!(stories.len(), 0);
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test --lib db`
Expected: FAIL — methods don't exist

**Step 3: Implement story CRUD**

Add to `impl Database`:

```rust
pub fn create_story(&self, title: &str, description: &str, epic_id: Option<i64>, status: Status, priority: Priority) -> Result<i64> {
    self.conn.execute(
        "INSERT INTO stories (title, description, epic_id, status, priority) VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params![title, description, epic_id, status_to_db(&status), priority_to_db(&priority)],
    )?;
    Ok(self.conn.last_insert_rowid())
}

pub fn list_stories_by_status(&self, status: Status, epic_id: Option<i64>) -> Result<Vec<Story>> {
    let (sql, params): (&str, Vec<Box<dyn rusqlite::types::ToSql>>) = match epic_id {
        Some(eid) => (
            "SELECT id, epic_id, title, description, status, priority, created_at, updated_at FROM stories WHERE status = ?1 AND epic_id = ?2 ORDER BY id",
            vec![Box::new(status_to_db(&status).to_string()), Box::new(eid)],
        ),
        None => (
            "SELECT id, epic_id, title, description, status, priority, created_at, updated_at FROM stories WHERE status = ?1 ORDER BY id",
            vec![Box::new(status_to_db(&status).to_string())],
        ),
    };
    let mut stmt = self.conn.prepare(sql)?;
    let stories = stmt.query_map(rusqlite::params_from_iter(params.iter()), |row| {
        let status_str: String = row.get(4)?;
        let priority_str: String = row.get(5)?;
        Ok(Story {
            id: row.get(0)?,
            epic_id: row.get(1)?,
            title: row.get(2)?,
            description: row.get(3)?,
            status: status_from_db(&status_str),
            priority: priority_from_db(&priority_str),
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
        })
    })?.collect::<Result<Vec<_>>>()?;
    Ok(stories)
}

pub fn update_story_status(&self, id: i64, status: Status) -> Result<()> {
    self.conn.execute(
        "UPDATE stories SET status = ?1, updated_at = datetime('now') WHERE id = ?2",
        rusqlite::params![status_to_db(&status), id],
    )?;
    Ok(())
}

pub fn delete_story(&self, id: i64) -> Result<()> {
    self.conn.execute("DELETE FROM stories WHERE id = ?1", [id])?;
    Ok(())
}
```

**Step 4: Run tests to verify they pass**

Run: `cargo test --lib db`
Expected: All 7 tests PASS

**Step 5: Commit**

```bash
git add src/db.rs
git commit -m "feat: add story CRUD operations"
```

---

### Task 6: Database Layer — Task CRUD

**Files:**
- Modify: `src/db.rs`

**Step 1: Write tests for task CRUD**

Add to the `tests` module:

```rust
#[test]
fn create_and_list_tasks() {
    let db = test_db();
    let sid = db.create_story("S1", "", None, Status::ToDo, Priority::Medium).unwrap();
    db.create_task(sid, "Write tests").unwrap();
    db.create_task(sid, "Implement feature").unwrap();

    let tasks = db.list_tasks(sid).unwrap();
    assert_eq!(tasks.len(), 2);
    assert_eq!(tasks[0].title, "Write tests");
    assert_eq!(tasks[0].sort_order, 0);
    assert_eq!(tasks[1].sort_order, 1);
    assert!(!tasks[0].done);
}

#[test]
fn toggle_task() {
    let db = test_db();
    let sid = db.create_story("S1", "", None, Status::ToDo, Priority::Medium).unwrap();
    let tid = db.create_task(sid, "Task 1").unwrap();

    db.toggle_task(tid).unwrap();
    let tasks = db.list_tasks(sid).unwrap();
    assert!(tasks[0].done);

    db.toggle_task(tid).unwrap();
    let tasks = db.list_tasks(sid).unwrap();
    assert!(!tasks[0].done);
}

#[test]
fn delete_task() {
    let db = test_db();
    let sid = db.create_story("S1", "", None, Status::ToDo, Priority::Medium).unwrap();
    let tid = db.create_task(sid, "Task 1").unwrap();
    db.delete_task(tid).unwrap();
    let tasks = db.list_tasks(sid).unwrap();
    assert_eq!(tasks.len(), 0);
}

#[test]
fn task_count_for_story() {
    let db = test_db();
    let sid = db.create_story("S1", "", None, Status::ToDo, Priority::Medium).unwrap();
    db.create_task(sid, "T1").unwrap();
    let tid2 = db.create_task(sid, "T2").unwrap();
    db.toggle_task(tid2).unwrap();

    let (done, total) = db.task_counts(sid).unwrap();
    assert_eq!(total, 2);
    assert_eq!(done, 1);
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test --lib db`
Expected: FAIL — methods don't exist

**Step 3: Implement task CRUD**

Add to `impl Database`:

```rust
pub fn create_task(&self, story_id: i64, title: &str) -> Result<i64> {
    let next_order: i32 = self.conn.query_row(
        "SELECT COALESCE(MAX(sort_order) + 1, 0) FROM tasks WHERE story_id = ?1",
        [story_id],
        |row| row.get(0),
    )?;
    self.conn.execute(
        "INSERT INTO tasks (story_id, title, sort_order) VALUES (?1, ?2, ?3)",
        rusqlite::params![story_id, title, next_order],
    )?;
    Ok(self.conn.last_insert_rowid())
}

pub fn list_tasks(&self, story_id: i64) -> Result<Vec<Task>> {
    let mut stmt = self.conn.prepare(
        "SELECT id, story_id, title, done, sort_order FROM tasks WHERE story_id = ?1 ORDER BY sort_order"
    )?;
    let tasks = stmt.query_map([story_id], |row| {
        Ok(Task {
            id: row.get(0)?,
            story_id: row.get(1)?,
            title: row.get(2)?,
            done: row.get(3)?,
            sort_order: row.get(4)?,
        })
    })?.collect::<Result<Vec<_>>>()?;
    Ok(tasks)
}

pub fn toggle_task(&self, id: i64) -> Result<()> {
    self.conn.execute(
        "UPDATE tasks SET done = NOT done WHERE id = ?1",
        [id],
    )?;
    Ok(())
}

pub fn delete_task(&self, id: i64) -> Result<()> {
    self.conn.execute("DELETE FROM tasks WHERE id = ?1", [id])?;
    Ok(())
}

pub fn task_counts(&self, story_id: i64) -> Result<(i32, i32)> {
    let total: i32 = self.conn.query_row(
        "SELECT COUNT(*) FROM tasks WHERE story_id = ?1",
        [story_id],
        |row| row.get(0),
    )?;
    let done: i32 = self.conn.query_row(
        "SELECT COUNT(*) FROM tasks WHERE story_id = ?1 AND done = 1",
        [story_id],
        |row| row.get(0),
    )?;
    Ok((done, total))
}
```

**Step 4: Run tests to verify they pass**

Run: `cargo test --lib db`
Expected: All 11 tests PASS

**Step 5: Commit**

```bash
git add src/db.rs
git commit -m "feat: add task CRUD operations"
```

---

### Task 7: App State & Actions

**Files:**
- Create: `src/app.rs`
- Create: `src/actions.rs`
- Modify: `src/main.rs` (add `mod app; mod actions;`)

**Step 1: Write tests for app state transitions**

In `src/app.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_state() {
        let app = App::new();
        assert_eq!(app.mode, Mode::Board);
        assert_eq!(app.selected_column, 0);
        assert!(app.epic_filter.is_none());
    }

    #[test]
    fn column_navigation_wraps() {
        let mut app = App::new();
        app.move_column_right();
        assert_eq!(app.selected_column, 1);
        app.selected_column = 3;
        app.move_column_right();
        assert_eq!(app.selected_column, 3); // stays at last
    }

    #[test]
    fn column_navigation_left_clamps() {
        let mut app = App::new();
        app.move_column_left();
        assert_eq!(app.selected_column, 0); // stays at first
    }

    #[test]
    fn mode_transitions() {
        let mut app = App::new();
        app.mode = Mode::Detail;
        assert_eq!(app.mode, Mode::Detail);
        app.mode = Mode::Board;
        assert_eq!(app.mode, Mode::Board);
    }
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test --lib app`
Expected: FAIL — module doesn't exist

**Step 3: Implement App state**

In `src/app.rs`:

```rust
use crate::models::{Epic, Story, Task, Status};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Board,
    Detail,
    EpicList,
    Input(InputTarget),
    Confirm(ConfirmAction),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputTarget {
    NewStory,
    NewTask,
    EditStoryTitle,
    EditStoryDescription,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfirmAction {
    DeleteStory,
    DeleteTask,
}

pub struct App {
    pub mode: Mode,
    pub selected_column: usize,
    pub selected_card: [usize; 4], // one per column
    pub selected_task: usize,
    pub epic_filter: Option<i64>,
    pub columns: [Vec<Story>; 4],
    pub epics: Vec<Epic>,
    pub tasks: Vec<Task>,
    pub current_story: Option<Story>,
    pub input_buffer: String,
    pub status_message: Option<String>,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        App {
            mode: Mode::Board,
            selected_column: 0,
            selected_card: [0; 4],
            selected_task: 0,
            epic_filter: None,
            columns: [vec![], vec![], vec![], vec![]],
            epics: vec![],
            tasks: vec![],
            current_story: None,
            input_buffer: String::new(),
            status_message: None,
            should_quit: false,
        }
    }

    pub fn move_column_right(&mut self) {
        if self.selected_column < 3 {
            self.selected_column += 1;
        }
    }

    pub fn move_column_left(&mut self) {
        if self.selected_column > 0 {
            self.selected_column -= 1;
        }
    }

    pub fn move_card_down(&mut self) {
        let col = self.selected_column;
        let len = self.columns[col].len();
        if len > 0 && self.selected_card[col] < len - 1 {
            self.selected_card[col] += 1;
        }
    }

    pub fn move_card_up(&mut self) {
        let col = self.selected_column;
        if self.selected_card[col] > 0 {
            self.selected_card[col] -= 1;
        }
    }

    pub fn selected_story(&self) -> Option<&Story> {
        let col = self.selected_column;
        self.columns[col].get(self.selected_card[col])
    }

    pub fn clamp_selections(&mut self) {
        for i in 0..4 {
            let len = self.columns[i].len();
            if len == 0 {
                self.selected_card[i] = 0;
            } else if self.selected_card[i] >= len {
                self.selected_card[i] = len - 1;
            }
        }
    }
}
```

In `src/actions.rs`:

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Quit,
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    MoveStoryLeft,
    MoveStoryRight,
    OpenDetail,
    CloseDetail,
    NewStory,
    NewTask,
    DeleteStory,
    DeleteTask,
    ToggleTask,
    EditStoryTitle,
    EditStoryDescription,
    OpenEpicList,
    SelectEpic(Option<i64>),
    ConfirmYes,
    ConfirmNo,
    InputChar(char),
    InputBackspace,
    InputConfirm,
    InputCancel,
}
```

Add `mod app; mod actions;` to `src/main.rs`.

**Step 4: Run tests to verify they pass**

Run: `cargo test --lib app`
Expected: All 4 tests PASS

**Step 5: Commit**

```bash
git add src/app.rs src/actions.rs src/main.rs
git commit -m "feat: add app state and action definitions"
```

---

### Task 8: Input Handling

**Files:**
- Create: `src/input.rs`
- Modify: `src/main.rs` (add `mod input;`)

**Step 1: Write tests for key mapping**

In `src/input.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyModifiers};

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }

    fn shift_key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::SHIFT)
    }

    #[test]
    fn board_navigation() {
        assert_eq!(handle_board_key(key(KeyCode::Char('h'))), Some(Action::MoveLeft));
        assert_eq!(handle_board_key(key(KeyCode::Char('l'))), Some(Action::MoveRight));
        assert_eq!(handle_board_key(key(KeyCode::Char('j'))), Some(Action::MoveDown));
        assert_eq!(handle_board_key(key(KeyCode::Char('k'))), Some(Action::MoveUp));
    }

    #[test]
    fn board_story_movement() {
        assert_eq!(handle_board_key(shift_key(KeyCode::Char('H'))), Some(Action::MoveStoryLeft));
        assert_eq!(handle_board_key(shift_key(KeyCode::Char('L'))), Some(Action::MoveStoryRight));
    }

    #[test]
    fn board_actions() {
        assert_eq!(handle_board_key(key(KeyCode::Enter)), Some(Action::OpenDetail));
        assert_eq!(handle_board_key(key(KeyCode::Char('n'))), Some(Action::NewStory));
        assert_eq!(handle_board_key(key(KeyCode::Char('q'))), Some(Action::Quit));
    }

    #[test]
    fn detail_navigation() {
        assert_eq!(handle_detail_key(key(KeyCode::Char('j'))), Some(Action::MoveDown));
        assert_eq!(handle_detail_key(key(KeyCode::Char(' '))), Some(Action::ToggleTask));
        assert_eq!(handle_detail_key(key(KeyCode::Esc)), Some(Action::CloseDetail));
    }

    #[test]
    fn input_mode() {
        assert_eq!(handle_input_key(key(KeyCode::Char('a'))), Some(Action::InputChar('a')));
        assert_eq!(handle_input_key(key(KeyCode::Enter)), Some(Action::InputConfirm));
        assert_eq!(handle_input_key(key(KeyCode::Esc)), Some(Action::InputCancel));
        assert_eq!(handle_input_key(key(KeyCode::Backspace)), Some(Action::InputBackspace));
    }
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test --lib input`
Expected: FAIL — module doesn't exist

**Step 3: Implement input handling**

In `src/input.rs`:

```rust
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crate::actions::Action;

pub fn handle_board_key(key: KeyEvent) -> Option<Action> {
    match (key.code, key.modifiers) {
        (KeyCode::Char('q'), _) => Some(Action::Quit),
        (KeyCode::Char('c'), KeyModifiers::CONTROL) => Some(Action::Quit),
        (KeyCode::Char('h'), KeyModifiers::NONE) => Some(Action::MoveLeft),
        (KeyCode::Char('l'), KeyModifiers::NONE) => Some(Action::MoveRight),
        (KeyCode::Char('j'), KeyModifiers::NONE) => Some(Action::MoveDown),
        (KeyCode::Char('k'), KeyModifiers::NONE) => Some(Action::MoveUp),
        (KeyCode::Char('H'), KeyModifiers::SHIFT) => Some(Action::MoveStoryLeft),
        (KeyCode::Char('L'), KeyModifiers::SHIFT) => Some(Action::MoveStoryRight),
        (KeyCode::Enter, _) => Some(Action::OpenDetail),
        (KeyCode::Char('n'), _) => Some(Action::NewStory),
        (KeyCode::Char('d'), _) => Some(Action::DeleteStory),
        (KeyCode::Char('e'), _) => Some(Action::OpenEpicList),
        _ => None,
    }
}

pub fn handle_detail_key(key: KeyEvent) -> Option<Action> {
    match key.code {
        KeyCode::Esc => Some(Action::CloseDetail),
        KeyCode::Char('j') => Some(Action::MoveDown),
        KeyCode::Char('k') => Some(Action::MoveUp),
        KeyCode::Char(' ') => Some(Action::ToggleTask),
        KeyCode::Char('n') => Some(Action::NewTask),
        KeyCode::Char('d') => Some(Action::DeleteTask),
        KeyCode::Char('e') => Some(Action::EditStoryTitle),
        KeyCode::Char('q') => Some(Action::Quit),
        _ => None,
    }
}

pub fn handle_epic_list_key(key: KeyEvent) -> Option<Action> {
    match key.code {
        KeyCode::Esc => Some(Action::CloseDetail),
        KeyCode::Char('j') => Some(Action::MoveDown),
        KeyCode::Char('k') => Some(Action::MoveUp),
        KeyCode::Char('q') => Some(Action::Quit),
        _ => None,
    }
}

pub fn handle_input_key(key: KeyEvent) -> Option<Action> {
    match key.code {
        KeyCode::Enter => Some(Action::InputConfirm),
        KeyCode::Esc => Some(Action::InputCancel),
        KeyCode::Backspace => Some(Action::InputBackspace),
        KeyCode::Char(c) => Some(Action::InputChar(c)),
        _ => None,
    }
}

pub fn handle_confirm_key(key: KeyEvent) -> Option<Action> {
    match key.code {
        KeyCode::Char('y') => Some(Action::ConfirmYes),
        KeyCode::Char('n') | KeyCode::Esc => Some(Action::ConfirmNo),
        _ => None,
    }
}
```

Add `mod input;` to `src/main.rs`.

**Step 4: Run tests to verify they pass**

Run: `cargo test --lib input`
Expected: All 5 tests PASS

**Step 5: Commit**

```bash
git add src/input.rs src/main.rs
git commit -m "feat: add keyboard input handling for all modes"
```

---

### Task 9: UI — Board View

**Files:**
- Create: `src/ui/mod.rs`
- Create: `src/ui/board.rs`
- Modify: `src/main.rs` (add `mod ui;`)

**Step 1: Implement board rendering**

This task is primarily visual rendering — testing TUI output is impractical. Instead we verify it compiles and renders without panic by running the app later in the integration task.

In `src/ui/mod.rs`:

```rust
mod board;
mod detail;
mod epic_list;

use ratatui::Frame;
use crate::app::{App, Mode};

pub fn render(app: &App, frame: &mut Frame) {
    match app.mode {
        Mode::Board => board::render(app, frame),
        Mode::Detail => detail::render(app, frame),
        Mode::EpicList => epic_list::render(app, frame),
        Mode::Input(_) => {
            // Render the underlying view, then overlay the input bar
            board::render(app, frame);
            render_input_bar(app, frame);
        }
        Mode::Confirm(_) => {
            board::render(app, frame);
            render_confirm_bar(app, frame);
        }
    }
}

fn render_input_bar(app: &App, frame: &mut Frame) {
    use ratatui::layout::{Constraint, Layout, Direction};
    use ratatui::widgets::{Block, Borders, Paragraph};
    use ratatui::style::{Style, Color};

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(frame.area());

    let input = Paragraph::new(app.input_buffer.as_str())
        .block(Block::default().borders(Borders::ALL).title("Input (Enter to confirm, Esc to cancel)"))
        .style(Style::default().fg(Color::Yellow));
    frame.render_widget(input, chunks[1]);
}

fn render_confirm_bar(app: &App, frame: &mut Frame) {
    use ratatui::layout::{Constraint, Layout, Direction};
    use ratatui::widgets::{Block, Borders, Paragraph};
    use ratatui::style::{Style, Color};

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(frame.area());

    let msg = match app.mode {
        Mode::Confirm(crate::app::ConfirmAction::DeleteStory) => "Delete this story? (y/n)",
        Mode::Confirm(crate::app::ConfirmAction::DeleteTask) => "Delete this task? (y/n)",
        _ => "Confirm? (y/n)",
    };

    let confirm = Paragraph::new(msg)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::Red));
    frame.render_widget(confirm, chunks[1]);
}
```

In `src/ui/board.rs`:

```rust
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use crate::app::App;
use crate::models::Status;

pub fn render(app: &App, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),  // header
            Constraint::Min(0),    // board
            Constraint::Length(1), // footer
        ])
        .split(frame.area());

    render_header(app, frame, chunks[0]);
    render_columns(app, frame, chunks[1]);
    render_footer(app, frame, chunks[2]);
}

fn render_header(app: &App, frame: &mut Frame, area: Rect) {
    let epic_name = match app.epic_filter {
        Some(eid) => app.epics.iter()
            .find(|e| e.id == eid)
            .map(|e| e.title.as_str())
            .unwrap_or("Unknown"),
        None => "All Epics",
    };
    let header = Paragraph::new(Line::from(vec![
        Span::styled(" Stack ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw("│ "),
        Span::styled(epic_name, Style::default().fg(Color::White)),
    ]));
    frame.render_widget(header, area);
}

fn render_columns(app: &App, frame: &mut Frame, area: Rect) {
    let col_constraints = vec![Constraint::Percentage(25); 4];
    let col_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(col_constraints)
        .split(area);

    for (i, status) in Status::all().iter().enumerate() {
        let is_selected_col = i == app.selected_column;
        let border_style = if is_selected_col {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let items: Vec<ListItem> = app.columns[i]
            .iter()
            .enumerate()
            .map(|(j, story)| {
                let is_selected = is_selected_col && j == app.selected_card[i];
                let style = if is_selected {
                    Style::default().fg(Color::Black).bg(Color::Cyan)
                } else {
                    Style::default()
                };
                let priority_color = match story.priority {
                    crate::models::Priority::Critical => Color::Red,
                    crate::models::Priority::High => Color::Yellow,
                    crate::models::Priority::Medium => Color::Blue,
                    crate::models::Priority::Low => Color::DarkGray,
                };
                let line = Line::from(vec![
                    Span::styled(
                        format!(" {} ", story.priority.as_str().chars().next().unwrap_or('?')),
                        Style::default().fg(priority_color),
                    ),
                    Span::styled(&story.title, style),
                ]);
                ListItem::new(line)
            })
            .collect();

        let count = app.columns[i].len();
        let title = format!("{} ({})", status.as_str(), count);
        let block = Block::default()
            .borders(Borders::ALL)
            .title(title)
            .border_style(border_style);

        let list = List::new(items).block(block);
        frame.render_widget(list, col_chunks[i]);
    }
}

fn render_footer(app: &App, frame: &mut Frame, area: Rect) {
    let hints = match app.mode {
        crate::app::Mode::Board => "j/k: nav  h/l: column  H/L: move story  Enter: open  n: new  d: delete  e: epics  q: quit",
        _ => "",
    };
    let msg = if let Some(ref status) = app.status_message {
        format!("{}", status)
    } else {
        hints.to_string()
    };
    let footer = Paragraph::new(Span::styled(msg, Style::default().fg(Color::DarkGray)));
    frame.render_widget(footer, area);
}
```

Add `mod ui;` to `src/main.rs`.

**Step 2: Verify it compiles**

Run: `cargo build`
Expected: Compiles (detail.rs and epic_list.rs don't exist yet — we'll stub them)

Create stub files first:

`src/ui/detail.rs`:
```rust
use ratatui::Frame;
use crate::app::App;

pub fn render(_app: &App, _frame: &mut Frame) {
    // TODO: implement in Task 10
}
```

`src/ui/epic_list.rs`:
```rust
use ratatui::Frame;
use crate::app::App;

pub fn render(_app: &App, _frame: &mut Frame) {
    // TODO: implement in Task 11
}
```

**Step 3: Verify it compiles**

Run: `cargo build`
Expected: Compiles successfully

**Step 4: Commit**

```bash
git add src/ui/ src/main.rs
git commit -m "feat: add board view rendering"
```

---

### Task 10: UI — Story Detail View

**Files:**
- Modify: `src/ui/detail.rs`

**Step 1: Implement detail view rendering**

Replace the stub in `src/ui/detail.rs`:

```rust
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use crate::app::App;

pub fn render(app: &App, frame: &mut Frame) {
    let story = match &app.current_story {
        Some(s) => s,
        None => return,
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),  // story info
            Constraint::Min(0),    // task list
            Constraint::Length(1), // footer
        ])
        .split(frame.area());

    // Story info
    let info = Paragraph::new(vec![
        Line::from(Span::styled(&story.title, Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan))),
        Line::from(Span::raw(&story.description)),
        Line::from(vec![
            Span::styled(format!("Status: {} ", story.status), Style::default().fg(Color::White)),
            Span::styled(format!("Priority: {}", story.priority), Style::default().fg(Color::Yellow)),
        ]),
    ])
    .block(Block::default().borders(Borders::ALL).title("Story"));
    frame.render_widget(info, chunks[0]);

    // Task checklist
    let items: Vec<ListItem> = app.tasks.iter().enumerate().map(|(i, task)| {
        let is_selected = i == app.selected_task;
        let check = if task.done { "[x]" } else { "[ ]" };
        let style = if is_selected {
            Style::default().fg(Color::Black).bg(Color::Cyan)
        } else if task.done {
            Style::default().fg(Color::DarkGray)
        } else {
            Style::default()
        };
        ListItem::new(Line::from(Span::styled(
            format!(" {} {}", check, task.title),
            style,
        )))
    }).collect();

    let done_count = app.tasks.iter().filter(|t| t.done).count();
    let total_count = app.tasks.len();
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!("Tasks ({}/{})", done_count, total_count));
    let list = List::new(items).block(block);
    frame.render_widget(list, chunks[1]);

    // Footer
    let footer = Paragraph::new(Span::styled(
        "j/k: nav  Space: toggle  n: new task  d: delete  e: edit  Esc: back",
        Style::default().fg(Color::DarkGray),
    ));
    frame.render_widget(footer, chunks[2]);
}
```

**Step 2: Verify it compiles**

Run: `cargo build`
Expected: Compiles successfully

**Step 3: Commit**

```bash
git add src/ui/detail.rs
git commit -m "feat: add story detail view rendering"
```

---

### Task 11: UI — Epic List View

**Files:**
- Modify: `src/ui/epic_list.rs`

**Step 1: Implement epic list rendering**

Replace the stub in `src/ui/epic_list.rs`:

```rust
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use crate::app::App;

pub fn render(app: &App, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),  // header
            Constraint::Min(0),    // list
            Constraint::Length(1), // footer
        ])
        .split(frame.area());

    let header = Paragraph::new(Span::styled(
        " Select Epic",
        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
    ));
    frame.render_widget(header, chunks[0]);

    let mut items: Vec<ListItem> = vec![];

    // "All Epics" option at the top
    let all_selected = app.epic_filter.is_none() && app.selected_task == 0;
    let all_style = if all_selected {
        Style::default().fg(Color::Black).bg(Color::Cyan)
    } else {
        Style::default()
    };
    items.push(ListItem::new(Line::from(Span::styled(" All Epics", all_style))));

    for (i, epic) in app.epics.iter().enumerate() {
        let is_selected = app.selected_task == i + 1;
        let style = if is_selected {
            Style::default().fg(Color::Black).bg(Color::Cyan)
        } else {
            Style::default()
        };
        items.push(ListItem::new(Line::from(Span::styled(
            format!(" {}", epic.title),
            style,
        ))));
    }

    let block = Block::default().borders(Borders::ALL).title("Epics");
    let list = List::new(items).block(block);
    frame.render_widget(list, chunks[1]);

    let footer = Paragraph::new(Span::styled(
        "j/k: nav  Enter: select  Esc: back",
        Style::default().fg(Color::DarkGray),
    ));
    frame.render_widget(footer, chunks[2]);
}
```

**Step 2: Verify it compiles**

Run: `cargo build`
Expected: Compiles successfully

**Step 3: Commit**

```bash
git add src/ui/epic_list.rs
git commit -m "feat: add epic list view rendering"
```

---

### Task 12: Main Event Loop — Wire Everything Together

**Files:**
- Modify: `src/main.rs`

This is the integration task that ties all modules together into a working application.

**Step 1: Implement main.rs**

Replace `src/main.rs` entirely:

```rust
mod actions;
mod app;
mod db;
mod input;
mod models;
mod ui;

use std::io;
use std::time::Duration;
use crossterm::event::{self, Event};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::execute;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use actions::Action;
use app::{App, ConfirmAction, InputTarget, Mode};
use db::Database;
use models::Status;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Resolve data directory
    let data_dir = dirs::data_dir()
        .expect("Could not determine data directory")
        .join("stack");
    std::fs::create_dir_all(&data_dir)?;

    // Open database
    let mut database = Database::open(&data_dir.join("stack.db"))?;
    database.migrate()?;

    // Setup terminal
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Init app state
    let mut app = App::new();
    refresh_board(&database, &mut app);

    // Main loop
    loop {
        terminal.draw(|frame| ui::render(&app, frame))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                let action = match app.mode {
                    Mode::Board => input::handle_board_key(key),
                    Mode::Detail => input::handle_detail_key(key),
                    Mode::EpicList => input::handle_epic_list_key(key),
                    Mode::Input(_) => input::handle_input_key(key),
                    Mode::Confirm(_) => input::handle_confirm_key(key),
                };

                if let Some(action) = action {
                    handle_action(&database, &mut app, action);
                }
            }
        }

        // Clear transient status messages
        if app.status_message.is_some() {
            // Simple approach: clear after next keypress (already handled by redraw)
        }

        if app.should_quit {
            break;
        }
    }

    // Restore terminal
    terminal::disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

fn handle_action(db: &Database, app: &mut App, action: Action) {
    app.status_message = None;

    match action {
        Action::Quit => app.should_quit = true,

        // Board navigation
        Action::MoveLeft => {
            if app.mode == Mode::Board {
                app.move_column_left();
            }
        }
        Action::MoveRight => {
            if app.mode == Mode::Board {
                app.move_column_right();
            }
        }
        Action::MoveUp => match app.mode {
            Mode::Board => app.move_card_up(),
            Mode::Detail => {
                if app.selected_task > 0 {
                    app.selected_task -= 1;
                }
            }
            Mode::EpicList => {
                if app.selected_task > 0 {
                    app.selected_task -= 1;
                }
            }
            _ => {}
        },
        Action::MoveDown => match app.mode {
            Mode::Board => app.move_card_down(),
            Mode::Detail => {
                if app.selected_task < app.tasks.len().saturating_sub(1) {
                    app.selected_task += 1;
                }
            }
            Mode::EpicList => {
                let max = app.epics.len(); // +1 for "All" option, -1 for zero index = len()
                if app.selected_task < max {
                    app.selected_task += 1;
                }
            }
            _ => {}
        },

        // Move story between columns
        Action::MoveStoryLeft => {
            if let Some(story) = app.selected_story() {
                if let Some(prev) = story.status.prev() {
                    let id = story.id;
                    if let Err(e) = db.update_story_status(id, prev) {
                        app.status_message = Some(format!("Error: {}", e));
                    }
                    refresh_board(db, app);
                }
            }
        }
        Action::MoveStoryRight => {
            if let Some(story) = app.selected_story() {
                if let Some(next) = story.status.next() {
                    let id = story.id;
                    if let Err(e) = db.update_story_status(id, next) {
                        app.status_message = Some(format!("Error: {}", e));
                    }
                    refresh_board(db, app);
                }
            }
        }

        // Open/close views
        Action::OpenDetail => {
            if let Some(story) = app.selected_story().cloned() {
                match db.list_tasks(story.id) {
                    Ok(tasks) => {
                        app.tasks = tasks;
                        app.current_story = Some(story);
                        app.selected_task = 0;
                        app.mode = Mode::Detail;
                    }
                    Err(e) => app.status_message = Some(format!("Error: {}", e)),
                }
            }
        }
        Action::CloseDetail => {
            app.mode = Mode::Board;
            app.current_story = None;
            app.tasks.clear();
            refresh_board(db, app);
        }
        Action::OpenEpicList => {
            match db.list_epics() {
                Ok(epics) => {
                    app.epics = epics;
                    app.selected_task = if app.epic_filter.is_none() {
                        0
                    } else {
                        app.epics.iter().position(|e| Some(e.id) == app.epic_filter)
                            .map(|i| i + 1).unwrap_or(0)
                    };
                    app.mode = Mode::EpicList;
                }
                Err(e) => app.status_message = Some(format!("Error: {}", e)),
            }
        }
        Action::SelectEpic(eid) => {
            app.epic_filter = eid;
            app.mode = Mode::Board;
            refresh_board(db, app);
        }

        // Input mode
        Action::NewStory => {
            app.input_buffer.clear();
            app.mode = Mode::Input(InputTarget::NewStory);
        }
        Action::NewTask => {
            app.input_buffer.clear();
            app.mode = Mode::Input(InputTarget::NewTask);
        }
        Action::EditStoryTitle => {
            if let Some(story) = &app.current_story {
                app.input_buffer = story.title.clone();
                app.mode = Mode::Input(InputTarget::EditStoryTitle);
            }
        }
        Action::EditStoryDescription => {}

        Action::InputChar(c) => app.input_buffer.push(c),
        Action::InputBackspace => { app.input_buffer.pop(); }
        Action::InputConfirm => {
            let text = app.input_buffer.clone();
            if !text.is_empty() {
                match app.mode {
                    Mode::Input(InputTarget::NewStory) => {
                        let status = Status::all()[app.selected_column];
                        if let Err(e) = db.create_story(&text, "", app.epic_filter, status, models::Priority::Medium) {
                            app.status_message = Some(format!("Error: {}", e));
                        }
                        app.mode = Mode::Board;
                        refresh_board(db, app);
                    }
                    Mode::Input(InputTarget::NewTask) => {
                        if let Some(story) = &app.current_story {
                            let sid = story.id;
                            if let Err(e) = db.create_task(sid, &text) {
                                app.status_message = Some(format!("Error: {}", e));
                            }
                            if let Ok(tasks) = db.list_tasks(sid) {
                                app.tasks = tasks;
                            }
                        }
                        app.mode = Mode::Detail;
                    }
                    Mode::Input(InputTarget::EditStoryTitle) => {
                        if let Some(story) = &app.current_story {
                            if let Err(e) = db.update_story_title(story.id, &text) {
                                app.status_message = Some(format!("Error: {}", e));
                            }
                            // Refresh current story
                            if let Ok(s) = db.get_story(story.id) {
                                app.current_story = Some(s);
                            }
                        }
                        app.mode = Mode::Detail;
                    }
                    _ => { app.mode = Mode::Board; }
                }
            } else {
                // Empty input, cancel
                app.mode = match app.mode {
                    Mode::Input(InputTarget::NewTask) | Mode::Input(InputTarget::EditStoryTitle) => Mode::Detail,
                    _ => Mode::Board,
                };
            }
            app.input_buffer.clear();
        }
        Action::InputCancel => {
            app.input_buffer.clear();
            app.mode = match app.mode {
                Mode::Input(InputTarget::NewTask) | Mode::Input(InputTarget::EditStoryTitle) | Mode::Input(InputTarget::EditStoryDescription) => Mode::Detail,
                _ => Mode::Board,
            };
        }

        // Confirm mode
        Action::DeleteStory => {
            if app.selected_story().is_some() {
                app.mode = Mode::Confirm(ConfirmAction::DeleteStory);
            }
        }
        Action::DeleteTask => {
            if !app.tasks.is_empty() {
                app.mode = Mode::Confirm(ConfirmAction::DeleteTask);
            }
        }
        Action::ConfirmYes => {
            match app.mode {
                Mode::Confirm(ConfirmAction::DeleteStory) => {
                    if let Some(story) = app.selected_story() {
                        let id = story.id;
                        if let Err(e) = db.delete_story(id) {
                            app.status_message = Some(format!("Error: {}", e));
                        }
                    }
                    app.mode = Mode::Board;
                    refresh_board(db, app);
                }
                Mode::Confirm(ConfirmAction::DeleteTask) => {
                    if let Some(task) = app.tasks.get(app.selected_task) {
                        let tid = task.id;
                        if let Err(e) = db.delete_task(tid) {
                            app.status_message = Some(format!("Error: {}", e));
                        }
                        if let Some(story) = &app.current_story {
                            if let Ok(tasks) = db.list_tasks(story.id) {
                                app.tasks = tasks;
                                if app.selected_task >= app.tasks.len() && app.selected_task > 0 {
                                    app.selected_task -= 1;
                                }
                            }
                        }
                    }
                    app.mode = Mode::Detail;
                }
                _ => { app.mode = Mode::Board; }
            }
        }
        Action::ConfirmNo => {
            app.mode = match app.mode {
                Mode::Confirm(ConfirmAction::DeleteTask) => Mode::Detail,
                _ => Mode::Board,
            };
        }

        // Task actions in detail view
        Action::ToggleTask => {
            if let Some(task) = app.tasks.get(app.selected_task) {
                let tid = task.id;
                if let Err(e) = db.toggle_task(tid) {
                    app.status_message = Some(format!("Error: {}", e));
                }
                if let Some(story) = &app.current_story {
                    if let Ok(tasks) = db.list_tasks(story.id) {
                        app.tasks = tasks;
                    }
                }
            }
        }
    }
}

fn refresh_board(db: &Database, app: &mut App) {
    for (i, status) in Status::all().iter().enumerate() {
        match db.list_stories_by_status(*status, app.epic_filter) {
            Ok(stories) => app.columns[i] = stories,
            Err(e) => app.status_message = Some(format!("Error loading {}: {}", status, e)),
        }
    }
    match db.list_epics() {
        Ok(epics) => app.epics = epics,
        Err(e) => app.status_message = Some(format!("Error loading epics: {}", e)),
    }
    app.clamp_selections();
}
```

Note: This requires two additional DB methods: `update_story_title` and `get_story`. Add to `src/db.rs` in `impl Database`:

```rust
pub fn update_story_title(&self, id: i64, title: &str) -> Result<()> {
    self.conn.execute(
        "UPDATE stories SET title = ?1, updated_at = datetime('now') WHERE id = ?2",
        rusqlite::params![title, id],
    )?;
    Ok(())
}

pub fn get_story(&self, id: i64) -> Result<Story> {
    self.conn.query_row(
        "SELECT id, epic_id, title, description, status, priority, created_at, updated_at FROM stories WHERE id = ?1",
        [id],
        |row| {
            let status_str: String = row.get(4)?;
            let priority_str: String = row.get(5)?;
            Ok(Story {
                id: row.get(0)?,
                epic_id: row.get(1)?,
                title: row.get(2)?,
                description: row.get(3)?,
                status: status_from_db(&status_str),
                priority: priority_from_db(&priority_str),
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        },
    )
}
```

Also need to handle `Enter` in epic list view to select an epic. Update `handle_epic_list_key` in `src/input.rs` to return `Action::OpenDetail` for Enter (we'll map it to `SelectEpic` in the handler), or add a dedicated path. Simpler: handle `Enter` in the epic list arm of `handle_action`. Add to `input.rs` `handle_epic_list_key`:

```rust
KeyCode::Enter => Some(Action::InputConfirm),  // reuse for epic selection
```

And in `handle_action`, add a special case for `InputConfirm` when in `EpicList` mode. Replace the `InputConfirm` handler with a check:

At the top of `Action::InputConfirm`, before the existing logic, add:

```rust
Action::InputConfirm => {
    if app.mode == Mode::EpicList {
        let eid = if app.selected_task == 0 {
            None
        } else {
            app.epics.get(app.selected_task - 1).map(|e| e.id)
        };
        app.epic_filter = eid;
        app.mode = Mode::Board;
        refresh_board(db, app);
        return;
    }
    // ... rest of InputConfirm logic
```

**Step 2: Verify it compiles and all tests pass**

Run: `cargo build && cargo test`
Expected: Compiles, all tests pass

**Step 3: Manually test the app**

Run: `cargo run`
Expected: Terminal shows kanban board with empty columns. Press `q` to quit. Board should render cleanly.

**Step 4: Commit**

```bash
git add src/
git commit -m "feat: wire up main event loop and complete TUI application"
```

---

### Task 13: Smoke Test & Polish

**Files:**
- Possibly minor adjustments to any file

**Step 1: Run the full test suite**

Run: `cargo test`
Expected: All tests pass

**Step 2: Run clippy for linting**

Run: `cargo clippy -- -D warnings`
Expected: No warnings

**Step 3: Fix any clippy issues**

Address any warnings from clippy (unused imports, unnecessary clones, etc.)

**Step 4: Run the app and do a manual walkthrough**

Run: `cargo run`

Manual test steps:
1. App starts with empty board — verify 4 columns render
2. Press `n`, type "My first story", press Enter — card appears in first column
3. Press `H`/`L` — story moves between columns
4. Press `Enter` — detail view opens
5. Press `n`, type "Subtask 1", Enter — task appears in checklist
6. Press `Space` — task toggles done
7. Press `Esc` — back to board
8. Press `q` — clean exit

**Step 5: Commit any fixes**

```bash
git add -A
git commit -m "chore: clippy fixes and polish"
```

---

## Summary

| Task | Description | Estimated Tests |
|------|-------------|-----------------|
| 1 | Add dependencies | 0 (build check) |
| 2 | Data models | 5 |
| 3 | DB schema & migrations | 2 |
| 4 | Epic CRUD | 2 |
| 5 | Story CRUD | 3 |
| 6 | Task CRUD | 4 |
| 7 | App state & actions | 4 |
| 8 | Input handling | 5 |
| 9 | Board view UI | 0 (visual) |
| 10 | Detail view UI | 0 (visual) |
| 11 | Epic list UI | 0 (visual) |
| 12 | Main event loop | 0 (integration) |
| 13 | Smoke test & polish | 0 (manual) |

**Total: 13 tasks, ~25 unit tests**
