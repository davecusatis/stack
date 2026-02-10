use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use crate::app::App;

pub fn render(app: &App, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(frame.area());

    let header = Paragraph::new(Span::styled(
        " Select Epic",
        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
    ));
    frame.render_widget(header, chunks[0]);

    let mut items: Vec<ListItem> = vec![];

    let all_selected = app.epic_filter.is_none() && app.list_selection == 0;
    let all_style = if all_selected {
        Style::default().fg(Color::Black).bg(Color::Cyan)
    } else {
        Style::default()
    };
    items.push(ListItem::new(Line::from(Span::styled(" All Epics", all_style))));

    for (i, epic) in app.epics.iter().enumerate() {
        let is_selected = app.list_selection == i + 1;
        let style = if is_selected {
            Style::default().fg(Color::Black).bg(Color::Cyan)
        } else {
            Style::default()
        };
        items.push(ListItem::new(Line::from(Span::styled(
            format!(" {}", epic.title),
            style,
        ))));
    }

    let block = Block::default().borders(Borders::ALL).title("Epics");
    let list = List::new(items).block(block);
    frame.render_widget(list, chunks[1]);

    let footer = Paragraph::new(Span::styled(
        "j/k: nav  Enter: select  Esc: back",
        Style::default().fg(Color::DarkGray),
    ));
    frame.render_widget(footer, chunks[2]);
}
