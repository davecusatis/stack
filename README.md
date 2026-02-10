# Stack

A terminal UI kanban board for personal task tracking, built in Rust.

Stack organizes work into **Epics** and **Stories** displayed on a four-column kanban board (To Do, In Progress, In Review, Done). Story bodies are markdown.

<img width="759" height="863" alt="Screenshot 2026-02-09 at 9 02 41 PM" src="https://github.com/user-attachments/assets/0471df3d-b9a1-4750-ab0a-8e4d562abb35" />
<img width="757" height="862" alt="Screenshot 2026-02-09 at 9 19 54 PM" src="https://github.com/user-attachments/assets/3b292e79-749e-4f8f-a182-edd3da7bf265" />
<img width="757" height="860" alt="Screenshot 2026-02-09 at 9 20 07 PM" src="https://github.com/user-attachments/assets/4f06edd6-1e7f-4dfc-a55b-14188a78080b" />


## Install

Requires Rust 1.85+ (2024 edition).

```
cargo install --path .
```

Or run directly:

```
cargo run
```

## Usage

### Board View

| Key | Action |
|-----|--------|
| `h` / `l` | Move between columns |
| `j` / `k` | Move between cards |
| `H` / `L` | Move story to previous/next column |
| `Enter` | Open story detail |
| `n` | New story |
| `d` | Delete story |
| `e` | Epic list |
| `q` | Quit |

### Story Detail View

| Key | Action |
|-----|--------|
| `j` / `k` | Scroll body |
| `e` | Edit title |
| `b` | Edit body (markdown) |
| `Esc` | Back to board |

### Epic List

| Key | Action |
|-----|--------|
| `j` / `k` | Navigate |
| `Enter` | Select epic filter |
| `Esc` | Back to board |

## Data Storage

SQLite database at `~/.local/share/stack/stack.db`. Created automatically on first run.

## Dependencies

- [ratatui](https://github.com/ratatui/ratatui) + [crossterm](https://github.com/crossterm-rs/crossterm) -- terminal rendering
- [rusqlite](https://github.com/rusqlite/rusqlite) (bundled) -- SQLite
- [dirs](https://github.com/dirs-dev/dirs-rs) -- XDG directory resolution
