mod actions;
mod app;
mod db;
mod input;
mod models;
mod ui;

use std::io;
use std::time::Duration;
use crossterm::event::{self, Event, KeyEventKind};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::execute;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use actions::Action;
use app::{App, ConfirmAction, InputTarget, Mode};
use db::Database;
use models::Status;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Resolve data directory
    let data_dir = dirs::data_dir()
        .expect("Could not determine data directory")
        .join("stack");
    std::fs::create_dir_all(&data_dir)?;

    // Open database
    let mut database = Database::open(&data_dir.join("stack.db"))?;
    database.migrate()?;

    // Setup terminal
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Init app state
    let mut app = App::new();
    refresh_board(&database, &mut app);

    // Main loop
    loop {
        terminal.draw(|frame| ui::render(&app, frame))?;

        if event::poll(Duration::from_millis(100))?
            && let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
        {
            let action = match app.mode {
                Mode::Board => input::handle_board_key(key),
                Mode::Detail => input::handle_detail_key(key),
                Mode::EpicList => input::handle_epic_list_key(key),
                Mode::Input(_) => input::handle_input_key(key),
                Mode::Confirm(_) => input::handle_confirm_key(key),
            };

            if let Some(action) = action {
                handle_action(&database, &mut app, action);
            }
        }

        if app.should_quit {
            break;
        }
    }

    // Restore terminal
    terminal::disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

fn handle_action(db: &Database, app: &mut App, action: Action) {
    app.status_message = None;

    match action {
        Action::Quit => app.should_quit = true,

        // Board navigation
        Action::MoveLeft => {
            if app.mode == Mode::Board {
                app.move_column_left();
            }
        }
        Action::MoveRight => {
            if app.mode == Mode::Board {
                app.move_column_right();
            }
        }
        Action::MoveUp => match app.mode {
            Mode::Board => app.move_card_up(),
            Mode::Detail => {
                app.scroll_offset = app.scroll_offset.saturating_sub(1);
            }
            Mode::EpicList => {
                if app.list_selection > 0 {
                    app.list_selection -= 1;
                }
            }
            _ => {}
        },
        Action::MoveDown => match app.mode {
            Mode::Board => app.move_card_down(),
            Mode::Detail => {
                app.scroll_offset = app.scroll_offset.saturating_add(1);
            }
            Mode::EpicList => {
                let max = app.epics.len();
                if app.list_selection < max {
                    app.list_selection += 1;
                }
            }
            _ => {}
        },

        // Move story between columns
        Action::MoveStoryLeft => {
            if let Some(story) = app.selected_story()
                && let Some(prev) = story.status.prev()
            {
                let id = story.id;
                if let Err(e) = db.update_story_status(id, prev) {
                    app.status_message = Some(format!("Error: {}", e));
                }
                refresh_board(db, app);
            }
        }
        Action::MoveStoryRight => {
            if let Some(story) = app.selected_story()
                && let Some(next) = story.status.next()
            {
                let id = story.id;
                if let Err(e) = db.update_story_status(id, next) {
                    app.status_message = Some(format!("Error: {}", e));
                }
                refresh_board(db, app);
            }
        }

        // Open/close views
        Action::OpenDetail => {
            if let Some(story) = app.selected_story().cloned() {
                app.current_story = Some(story);
                app.scroll_offset = 0;
                app.mode = Mode::Detail;
            }
        }
        Action::CloseDetail => {
            app.mode = Mode::Board;
            app.current_story = None;
            refresh_board(db, app);
        }
        Action::OpenEpicList => {
            match db.list_epics() {
                Ok(epics) => {
                    app.epics = epics;
                    app.list_selection = if app.epic_filter.is_none() {
                        0
                    } else {
                        app.epics.iter().position(|e| Some(e.id) == app.epic_filter)
                            .map(|i| i + 1).unwrap_or(0)
                    };
                    app.mode = Mode::EpicList;
                }
                Err(e) => app.status_message = Some(format!("Error: {}", e)),
            }
        }

        // Input mode
        Action::NewStory => {
            app.input_buffer.clear();
            app.mode = Mode::Input(InputTarget::NewStory);
        }
        Action::EditStoryTitle => {
            if let Some(story) = &app.current_story {
                app.input_buffer = story.title.clone();
                app.mode = Mode::Input(InputTarget::EditStoryTitle);
            }
        }
        Action::EditStoryBody => {
            if let Some(story) = &app.current_story {
                app.input_buffer = story.description.clone();
                app.mode = Mode::Input(InputTarget::EditStoryBody);
            }
        }
        Action::InputChar(c) => app.input_buffer.push(c),
        Action::InputBackspace => { app.input_buffer.pop(); }
        Action::InputConfirm => {
            // Handle epic list selection via InputConfirm
            if app.mode == Mode::EpicList {
                let eid = if app.list_selection == 0 {
                    None
                } else {
                    app.epics.get(app.list_selection - 1).map(|e| e.id)
                };
                app.epic_filter = eid;
                app.mode = Mode::Board;
                refresh_board(db, app);
                return;
            }

            let text = app.input_buffer.clone();
            if !text.is_empty() {
                match app.mode {
                    Mode::Input(InputTarget::NewStory) => {
                        let status = Status::all()[app.selected_column];
                        if let Err(e) = db.create_story(&text, "", app.epic_filter, status, models::Priority::Medium) {
                            app.status_message = Some(format!("Error: {}", e));
                        }
                        app.mode = Mode::Board;
                        refresh_board(db, app);
                    }
                    Mode::Input(InputTarget::EditStoryTitle) => {
                        if let Some(story) = &app.current_story {
                            if let Err(e) = db.update_story_title(story.id, &text) {
                                app.status_message = Some(format!("Error: {}", e));
                            }
                            if let Ok(s) = db.get_story(story.id) {
                                app.current_story = Some(s);
                            }
                        }
                        app.mode = Mode::Detail;
                    }
                    Mode::Input(InputTarget::EditStoryBody) => {
                        if let Some(story) = &app.current_story {
                            if let Err(e) = db.update_story_description(story.id, &text) {
                                app.status_message = Some(format!("Error: {}", e));
                            }
                            if let Ok(s) = db.get_story(story.id) {
                                app.current_story = Some(s);
                            }
                        }
                        app.mode = Mode::Detail;
                    }
                    _ => { app.mode = Mode::Board; }
                }
            } else {
                app.mode = match app.mode {
                    Mode::Input(InputTarget::EditStoryTitle) | Mode::Input(InputTarget::EditStoryBody) => Mode::Detail,
                    _ => Mode::Board,
                };
            }
            app.input_buffer.clear();
        }
        Action::InputCancel => {
            app.input_buffer.clear();
            app.mode = match app.mode {
                Mode::Input(InputTarget::EditStoryTitle) | Mode::Input(InputTarget::EditStoryBody) => Mode::Detail,
                _ => Mode::Board,
            };
        }

        // Confirm mode
        Action::DeleteStory => {
            if app.selected_story().is_some() {
                app.mode = Mode::Confirm(ConfirmAction::DeleteStory);
            }
        }
        Action::ConfirmYes => {
            if let Mode::Confirm(ConfirmAction::DeleteStory) = app.mode {
                if let Some(story) = app.selected_story() {
                    let id = story.id;
                    if let Err(e) = db.delete_story(id) {
                        app.status_message = Some(format!("Error: {}", e));
                    }
                }
                app.mode = Mode::Board;
                refresh_board(db, app);
            }
        }
        Action::ConfirmNo => {
            app.mode = Mode::Board;
        }
    }
}

fn refresh_board(db: &Database, app: &mut App) {
    for (i, status) in Status::all().iter().enumerate() {
        match db.list_stories_by_status(*status, app.epic_filter) {
            Ok(stories) => app.columns[i] = stories,
            Err(e) => app.status_message = Some(format!("Error loading {}: {}", status, e)),
        }
    }
    match db.list_epics() {
        Ok(epics) => app.epics = epics,
        Err(e) => app.status_message = Some(format!("Error loading epics: {}", e)),
    }
    app.clamp_selections();
}
