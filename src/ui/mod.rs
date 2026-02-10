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

fn centered_dialog(frame: &mut Frame, width: u16, height: u16) -> ratatui::layout::Rect {
    use ratatui::layout::{Constraint, Layout, Direction};

    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Fill(1), Constraint::Length(height), Constraint::Fill(1)])
        .split(frame.area());

    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Fill(1), Constraint::Length(width), Constraint::Fill(1)])
        .split(vertical[1]);

    horizontal[1]
}

fn render_input_bar(app: &App, frame: &mut Frame) {
    use ratatui::widgets::{Block, Borders, Paragraph, Clear};
    use ratatui::style::{Style, Color};

    let area = centered_dialog(frame, 50, 3);

    frame.render_widget(Clear, area);
    let input = Paragraph::new(app.input_buffer.as_str())
        .block(Block::default().borders(Borders::ALL).title("Input (Enter to confirm, Esc to cancel)").style(Style::default().bg(Color::Black).fg(Color::White)))
        .style(Style::default().fg(Color::Yellow).bg(Color::Black));
    frame.render_widget(input, area);
}

fn render_confirm_bar(app: &App, frame: &mut Frame) {
    use ratatui::widgets::{Block, Borders, Paragraph, Clear};
    use ratatui::style::{Style, Color};

    let area = centered_dialog(frame, 40, 3);

    let msg = match app.mode {
        Mode::Confirm(crate::app::ConfirmAction::DeleteStory) => "Delete this story? (y/n)",
        _ => "Confirm? (y/n)",
    };

    frame.render_widget(Clear, area);
    let confirm = Paragraph::new(msg)
        .block(Block::default().borders(Borders::ALL).style(Style::default().bg(Color::Black).fg(Color::White)))
        .style(Style::default().fg(Color::Red).bg(Color::Black));
    frame.render_widget(confirm, area);
}
