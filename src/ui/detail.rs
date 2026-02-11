use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use tui_md;
use crate::app::App;

pub fn render(app: &App, frame: &mut Frame) {
    let story = match &app.current_story {
        Some(s) => s,
        None => return,
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(frame.area());

    // Story header
    let info = Paragraph::new(vec![
        Line::from(Span::styled(&story.title, Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan))),
        Line::from(vec![
            Span::styled(format!("Status: {} ", story.status), Style::default().fg(Color::White)),
            Span::styled(format!("Priority: {}", story.priority), Style::default().fg(Color::Yellow)),
        ]),
    ])
    .block(Block::default().borders(Borders::ALL).title("Story"));
    frame.render_widget(info, chunks[0]);

    // Markdown body
    let body_lines: Vec<Line> = if story.description.is_empty() {
        vec![Line::from(Span::styled(
            "(no description — press 'b' to add)",
            Style::default().fg(Color::DarkGray),
        ))]
    } else {
        tui_md::render(&story.description)
    };

    let body = Paragraph::new(body_lines)
        .block(Block::default().borders(Borders::ALL).title("Body"))
        .wrap(Wrap { trim: false })
        .scroll((app.scroll_offset, 0));
    frame.render_widget(body, chunks[1]);

    // Footer
    let footer = Paragraph::new(Span::styled(
        "j/k/↑↓: scroll  e: edit title  b: edit body  Esc: back",
        Style::default().fg(Color::DarkGray),
    ));
    frame.render_widget(footer, chunks[2]);
}
