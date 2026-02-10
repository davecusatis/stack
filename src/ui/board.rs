use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use crate::app::App;
use crate::models::Status;

pub fn render(app: &App, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(frame.area());

    render_header(app, frame, chunks[0]);
    render_columns(app, frame, chunks[1]);
    render_footer(app, frame, chunks[2]);
}

fn render_header(app: &App, frame: &mut Frame, area: Rect) {
    let epic_name = match app.epic_filter {
        Some(eid) => app.epics.iter()
            .find(|e| e.id == eid)
            .map(|e| e.title.as_str())
            .unwrap_or("Unknown"),
        None => "All Epics",
    };
    let header = Paragraph::new(Line::from(vec![
        Span::styled(" Stack ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw("│ "),
        Span::styled(epic_name, Style::default().fg(Color::White)),
    ]));
    frame.render_widget(header, area);
}

fn render_columns(app: &App, frame: &mut Frame, area: Rect) {
    let col_constraints = vec![Constraint::Percentage(25); 4];
    let col_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(col_constraints)
        .split(area);

    for (i, status) in Status::all().iter().enumerate() {
        let is_selected_col = i == app.selected_column;
        let border_style = if is_selected_col {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let items: Vec<ListItem> = app.columns[i]
            .iter()
            .enumerate()
            .map(|(j, story)| {
                let is_selected = is_selected_col && j == app.selected_card[i];
                let priority_color = match story.priority {
                    crate::models::Priority::Critical => Color::Red,
                    crate::models::Priority::High => Color::Yellow,
                    crate::models::Priority::Medium => Color::Blue,
                    crate::models::Priority::Low => Color::DarkGray,
                };
                let (pri_style, title_style) = if is_selected {
                    (
                        Style::default().fg(priority_color).bg(Color::Cyan),
                        Style::default().fg(Color::Black).bg(Color::Cyan),
                    )
                } else {
                    (
                        Style::default().fg(priority_color),
                        Style::default(),
                    )
                };
                let line = Line::from(vec![
                    Span::styled(
                        format!(" {} ", story.priority.as_str().chars().next().unwrap_or('?')),
                        pri_style,
                    ),
                    Span::styled(&story.title, title_style),
                ]);
                let mut item = ListItem::new(line);
                if is_selected {
                    item = item.style(Style::default().bg(Color::Cyan));
                }
                item
            })
            .collect();

        let count = app.columns[i].len();
        let title = format!("{} ({})", status.as_str(), count);
        let block = Block::default()
            .borders(Borders::ALL)
            .title(title)
            .border_style(border_style);

        let list = List::new(items).block(block);
        frame.render_widget(list, col_chunks[i]);
    }
}

fn render_footer(app: &App, frame: &mut Frame, area: Rect) {
    let hints = match app.mode {
        crate::app::Mode::Board => "j/k: nav  h/l: column  H/L: move story  Enter: open  n: new  d: delete  e: epics  q: quit",
        _ => "",
    };
    let msg = if let Some(ref status) = app.status_message {
        status.to_string()
    } else {
        hints.to_string()
    };
    let footer = Paragraph::new(Span::styled(msg, Style::default().fg(Color::DarkGray)));
    frame.render_widget(footer, area);
}
