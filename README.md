# Stack

A terminal UI kanban board for personal task tracking, built in Rust.

Stack organizes work into **Epics** and **Stories** displayed on a four-column kanban board (To Do, In Progress, In Review, Done). Story bodies are markdown.

<img width="1090" height="861" alt="Screenshot 2026-02-09 at 9 33 26 PM" src="https://github.com/user-attachments/assets/f5ca9478-5bef-47ec-95ad-56f8980856dd" />

## Install

### Homebrew

```
brew tap davecusatis/tap
brew install stack
```

### From source

Requires Rust 1.85+ (2024 edition).

```
cargo install --path .
```

## Usage

### Board View

| Key | Action |
|-----|--------|
| `←` / `→` | Move between columns |
| `↑` / `↓` | Move between cards |
| `a` / `s` | Move story to previous/next status |
| `Enter` | Open story detail |
| `n` | New story |
| `d` | Delete story |
| `e` | Epic list |
| `q` | Quit |

### Story Detail View

| Key | Action |
|-----|--------|
| `↑` / `↓` | Scroll body |
| `e` | Edit title |
| `b` | Edit body (markdown) |
| `Esc` | Back to board |

### Epic List

| Key | Action |
|-----|--------|
| `↑` / `↓` | Navigate |
| `Enter` | Select epic filter |
| `Esc` | Back to board |

## Data Storage

SQLite database at `~/.local/share/stack/stack.db`. Created automatically on first run.

## Dependencies

- [ratatui](https://github.com/ratatui/ratatui) + [crossterm](https://github.com/crossterm-rs/crossterm) -- terminal rendering
- [rusqlite](https://github.com/rusqlite/rusqlite) (bundled) -- SQLite
- [dirs](https://github.com/dirs-dev/dirs-rs) -- XDG directory resolution
