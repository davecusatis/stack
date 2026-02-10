use crate::models::{Epic, Story};

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
    EditStoryTitle,
    EditStoryBody,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfirmAction {
    DeleteStory,
}

pub struct App {
    pub mode: Mode,
    pub selected_column: usize,
    pub selected_card: [usize; 4],
    pub list_selection: usize,
    pub scroll_offset: u16,
    pub epic_filter: Option<i64>,
    pub columns: [Vec<Story>; 4],
    pub epics: Vec<Epic>,
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
            list_selection: 0,
            scroll_offset: 0,
            epic_filter: None,
            columns: [vec![], vec![], vec![], vec![]],
            epics: vec![],
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
        assert_eq!(app.selected_column, 3);
    }

    #[test]
    fn column_navigation_left_clamps() {
        let mut app = App::new();
        app.move_column_left();
        assert_eq!(app.selected_column, 0);
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
