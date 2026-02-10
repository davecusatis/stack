# Vim Visual Line Mode in Preview Pane

## Overview

Add vim-like linewise visual mode (`V`) to the TUI preview pane, allowing users to select lines and yank them to the system clipboard and an internal register.

## Entering and Using Visual Mode

When the preview pane is focused, pressing `V` enters linewise visual mode. The current cursor line becomes the anchor.

**Flow:**
1. Preview is focused, user presses `V`
2. Current line is highlighted and becomes the **anchor**
3. `j`/`k` extend the selection up/down from the anchor (anchor stays fixed, cursor moves)
4. `y` yanks selected lines to system clipboard + internal register, exits visual mode
5. `Esc` cancels visual mode, clears selection

Status bar shows `-- VISUAL LINE --` while in this mode.

**Prerequisite:** The preview pane needs a cursor concept (highlighted current line) when focused. `j`/`k` move this cursor line-by-line, scrolling the view when it reaches the edges. `V` anchors at the cursor position.

## Architecture & Components

### New State

- `preview_cursor: usize` — current cursor line within preview content
- `visual_anchor: Option<usize>` — line where `V` was pressed (`None` when not in visual mode)
- `yank_register: Option<String>` — internal register holding last yanked text

### New Mode

Add `VisualLine` variant to the existing mode enum.

### File Changes

- `tui/mod.rs` — add `VisualLine` mode, new state fields, cursor logic
- `tui/events.rs` — handle `V`, `j`/`k` in visual mode, `y` to yank, `Esc` to cancel
- `tui/ui.rs` — render cursor highlight when preview focused, render selection highlight in visual mode, show `-- VISUAL LINE --` in status bar

### New Dependency

- `arboard` — cross-platform system clipboard access

### Cursor Behavior (Preview Focused, Normal Mode)

- `j`/`k` move cursor line-by-line (replaces current scroll-only behavior)
- `Ctrl+d`/`Ctrl+u` move cursor by half page
- `gg`/`G` move cursor to top/bottom
- View scrolls to keep cursor visible

## Visual Highlight & Yank Behavior

### Selection Range

Always `min(anchor, cursor)..=max(anchor, cursor)` — works regardless of selection direction.

### Highlight Rendering

- **Visual mode:** Selected lines get a muted blue/purple background. Cursor line gets a slightly brighter variant.
- **Normal mode (preview focused):** Cursor line gets a subtle background highlight (like vim's `CursorLine`).

### Yank Behavior

1. Collect text of selected lines from the **raw markdown source** (not rendered/styled version)
2. Join with newlines
3. Copy to system clipboard via `arboard`
4. Store in internal `yank_register`
5. Exit visual mode, clear selection
6. Status bar shows `"N lines yanked"` briefly

## Edge Cases & Constraints

- **Empty preview:** `V` does nothing if no note selected or note is empty
- **Summary vs note toggle:** Cursor and selection reset when toggling views. Yank grabs from whichever view is active (note or summary raw markdown).
- **Scroll boundaries:** Cursor clamps to `0..content_line_count - 1`. Viewport scrolls to keep cursor visible.
- **Clipboard failure:** Still store in internal register, show `"yanked N lines (clipboard unavailable)"` in status bar.
- **Mode transitions:** Entering command mode (`:`), search mode (`/`), or switching focus away from preview exits visual mode and clears selection.
