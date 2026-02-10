use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crate::actions::Action;

pub fn handle_board_key(key: KeyEvent) -> Option<Action> {
    match key.code {
        KeyCode::Char('q') => Some(Action::Quit),
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => Some(Action::Quit),
        KeyCode::Char('a') => Some(Action::MoveStoryLeft),
        KeyCode::Char('s') => Some(Action::MoveStoryRight),
        KeyCode::Left => Some(Action::MoveLeft),
        KeyCode::Right => Some(Action::MoveRight),
        KeyCode::Down => Some(Action::MoveDown),
        KeyCode::Up => Some(Action::MoveUp),
        KeyCode::Enter => Some(Action::OpenDetail),
        KeyCode::Char('n') => Some(Action::NewStory),
        KeyCode::Char('d') => Some(Action::DeleteStory),
        KeyCode::Char('e') => Some(Action::OpenEpicList),
        _ => None,
    }
}

pub fn handle_detail_key(key: KeyEvent) -> Option<Action> {
    match key.code {
        KeyCode::Esc => Some(Action::CloseDetail),
        KeyCode::Down => Some(Action::MoveDown),
        KeyCode::Up => Some(Action::MoveUp),
        KeyCode::Char('e') => Some(Action::EditStoryTitle),
        KeyCode::Char('b') => Some(Action::EditStoryBody),
        KeyCode::Char('q') => Some(Action::Quit),
        _ => None,
    }
}

pub fn handle_epic_list_key(key: KeyEvent) -> Option<Action> {
    match key.code {
        KeyCode::Esc => Some(Action::CloseDetail),
        KeyCode::Down => Some(Action::MoveDown),
        KeyCode::Up => Some(Action::MoveUp),
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

    #[test]
    fn board_navigation() {
        assert_eq!(handle_board_key(key(KeyCode::Left)), Some(Action::MoveLeft));
        assert_eq!(handle_board_key(key(KeyCode::Right)), Some(Action::MoveRight));
        assert_eq!(handle_board_key(key(KeyCode::Down)), Some(Action::MoveDown));
        assert_eq!(handle_board_key(key(KeyCode::Up)), Some(Action::MoveUp));
    }

    #[test]
    fn board_story_movement() {
        assert_eq!(handle_board_key(key(KeyCode::Char('a'))), Some(Action::MoveStoryLeft));
        assert_eq!(handle_board_key(key(KeyCode::Char('s'))), Some(Action::MoveStoryRight));
    }

    #[test]
    fn board_actions() {
        assert_eq!(handle_board_key(key(KeyCode::Enter)), Some(Action::OpenDetail));
        assert_eq!(handle_board_key(key(KeyCode::Char('n'))), Some(Action::NewStory));
        assert_eq!(handle_board_key(key(KeyCode::Char('q'))), Some(Action::Quit));
    }

    #[test]
    fn detail_navigation() {
        assert_eq!(handle_detail_key(key(KeyCode::Down)), Some(Action::MoveDown));
        assert_eq!(handle_detail_key(key(KeyCode::Up)), Some(Action::MoveUp));
        assert_eq!(handle_detail_key(key(KeyCode::Char('e'))), Some(Action::EditStoryTitle));
        assert_eq!(handle_detail_key(key(KeyCode::Char('b'))), Some(Action::EditStoryBody));
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
