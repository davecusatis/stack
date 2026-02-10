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
        KeyCode::Enter => Some(Action::InputConfirm),
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
