use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap};
use crate::app::App;
use crate::models::Status;

pub fn render(app: &App, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(0),
            Constraint::Length(8),
            Constraint::Length(1),
        ])
        .split(frame.area());

    render_header(app, frame, chunks[0]);
    render_columns(app, frame, chunks[1]);
    render_preview(app, frame, chunks[2]);
    render_footer(app, frame, chunks[3]);
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
            .map(|story| {
                let priority_color = match story.priority {
                    crate::models::Priority::Critical => Color::Red,
                    crate::models::Priority::High => Color::Yellow,
                    crate::models::Priority::Medium => Color::Blue,
                    crate::models::Priority::Low => Color::DarkGray,
                };
                let line = Line::from(vec![
                    Span::styled(
                        format!(" {} ", story.priority.as_str().chars().next().unwrap_or('?')),
                        Style::default().fg(priority_color),
                    ),
                    Span::raw(&story.title),
                ]);
                ListItem::new(line)
            })
            .collect();

        let count = app.columns[i].len();
        let title = format!("{} ({})", status.as_str(), count);
        let block = Block::default()
            .borders(Borders::ALL)
            .title(title)
            .border_style(border_style);

        let highlight_style = if is_selected_col {
            Style::default().fg(Color::Black).bg(Color::Cyan)
        } else {
            Style::default()
        };

        let list = List::new(items)
            .block(block)
            .highlight_style(highlight_style);

        let mut state = ListState::default();
        if is_selected_col && !app.columns[i].is_empty() {
            state.select(Some(app.selected_card[i]));
        }

        frame.render_stateful_widget(list, col_chunks[i], &mut state);
    }
}

fn render_preview(app: &App, frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Preview")
        .border_style(Style::default().fg(Color::DarkGray));

    match app.selected_story() {
        Some(story) => {
            let mut lines: Vec<Line> = Vec::new();

            lines.push(Line::from(Span::styled(
                &story.title,
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            )));

            if story.description.is_empty() {
                lines.push(Line::from(Span::styled(
                    "(no description)",
                    Style::default().fg(Color::DarkGray),
                )));
            } else {
                for line in story.description.lines() {
                    if let Some(heading) = line.strip_prefix("# ") {
                        lines.push(Line::from(Span::styled(heading, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))));
                    } else if let Some(heading) = line.strip_prefix("## ") {
                        lines.push(Line::from(Span::styled(heading, Style::default().fg(Color::Cyan))));
                    } else if let Some(heading) = line.strip_prefix("### ") {
                        lines.push(Line::from(Span::styled(heading, Style::default().fg(Color::Cyan).add_modifier(Modifier::ITALIC))));
                    } else if let Some(item) = line.strip_prefix("- ") {
                        lines.push(Line::from(vec![
                            Span::styled("  • ", Style::default().fg(Color::DarkGray)),
                            Span::raw(item),
                        ]));
                    } else if line.starts_with("```") {
                        lines.push(Line::from(Span::styled(line, Style::default().fg(Color::DarkGray))));
                    } else {
                        lines.push(Line::from(Span::raw(line)));
                    }
                }
            }

            let paragraph = Paragraph::new(lines)
                .block(block)
                .wrap(Wrap { trim: false });
            frame.render_widget(paragraph, area);
        }
        None => {
            let empty = Paragraph::new(Span::styled(
                "No story selected",
                Style::default().fg(Color::DarkGray),
            ))
            .block(block);
            frame.render_widget(empty, area);
        }
    }
}

fn render_footer(app: &App, frame: &mut Frame, area: Rect) {
    let hints = match app.mode {
        crate::app::Mode::Board => "hjkl/↑↓←→: nav  a/s: move story  Enter: open  n: new  d: delete  e: epics  q: quit",
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
