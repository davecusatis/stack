use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
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

    let info = Paragraph::new(vec![
        Line::from(Span::styled(&story.title, Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan))),
        Line::from(Span::raw(&story.description)),
        Line::from(vec![
            Span::styled(format!("Status: {} ", story.status), Style::default().fg(Color::White)),
            Span::styled(format!("Priority: {}", story.priority), Style::default().fg(Color::Yellow)),
        ]),
    ])
    .block(Block::default().borders(Borders::ALL).title("Story"));
    frame.render_widget(info, chunks[0]);

    let items: Vec<ListItem> = app.tasks.iter().enumerate().map(|(i, task)| {
        let is_selected = i == app.selected_task;
        let check = if task.done { "[x]" } else { "[ ]" };
        let style = if is_selected {
            Style::default().fg(Color::Black).bg(Color::Cyan)
        } else if task.done {
            Style::default().fg(Color::DarkGray)
        } else {
            Style::default()
        };
        ListItem::new(Line::from(Span::styled(
            format!(" {} {}", check, task.title),
            style,
        )))
    }).collect();

    let done_count = app.tasks.iter().filter(|t| t.done).count();
    let total_count = app.tasks.len();
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!("Tasks ({}/{})", done_count, total_count));
    let list = List::new(items).block(block);
    frame.render_widget(list, chunks[1]);

    let footer = Paragraph::new(Span::styled(
        "j/k: nav  Space: toggle  n: new task  d: delete  e: edit  Esc: back",
        Style::default().fg(Color::DarkGray),
    ));
    frame.render_widget(footer, chunks[2]);
}
