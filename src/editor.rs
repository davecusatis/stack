use std::env;
use std::error::Error;
use std::fs;
use std::io;
use std::process::Command;

use crossterm::execute;
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};

pub fn resolve_editor() -> String {
    env::var("EDITOR").unwrap_or_else(|_| "vim".to_string())
}

pub fn spawn_editor(current: &str) -> Result<Option<String>, Box<dyn Error>> {
    // Write current content to temp file
    let dir = env::temp_dir();
    let path = dir.join("stack_edit.md");
    fs::write(&path, current)?;

    // Suspend TUI
    terminal::disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;

    // Spawn editor
    let editor = resolve_editor();
    let status = Command::new(&editor)
        .arg(&path)
        .status();

    // Resume TUI (always, even on error)
    execute!(io::stdout(), EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    match status {
        Ok(s) if s.success() => {
            let content = fs::read_to_string(&path)?;
            fs::remove_file(&path).ok();
            Ok(Some(content))
        }
        Ok(_) => {
            fs::remove_file(&path).ok();
            Ok(None)
        }
        Err(e) => {
            fs::remove_file(&path).ok();
            Err(format!("Failed to launch editor '{}': {}", editor, e).into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_editor_uses_env_var() {
        // SAFETY: test is run single-threaded via --test-threads=1
        unsafe { env::set_var("EDITOR", "nvim") };
        assert_eq!(resolve_editor(), "nvim");
        unsafe { env::remove_var("EDITOR") };
    }

    #[test]
    fn resolve_editor_falls_back_to_vim() {
        // SAFETY: test is run single-threaded via --test-threads=1
        unsafe { env::remove_var("EDITOR") };
        assert_eq!(resolve_editor(), "vim");
    }
}
