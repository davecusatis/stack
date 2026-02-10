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
