use std::collections::HashMap;

use crate::cli::{Command, EpicAction, StoryAction, TaskAction};
use crate::db::Database;
use crate::models::Status;

pub fn run(command: Command, db: &Database) -> Result<(), Box<dyn std::error::Error>> {
    let result = match command {
        Command::Epic { action } => handle_epic(action, db)?,
        Command::Story { action } => handle_story(action, db)?,
        Command::Task { action } => handle_task(action, db)?,
        Command::Board { epic } => handle_board(epic, db)?,
    };
    println!("{}", serde_json::to_string(&serde_json::json!({ "result": result }))?);
    Ok(())
}

fn handle_epic(action: EpicAction, db: &Database) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    match action {
        EpicAction::Create { title, description, color } => {
            let id = db.create_epic(&title, &description, &color)?;
            let epics = db.list_epics()?;
            let epic = epics.into_iter().find(|e| e.id == id)
                .ok_or("Failed to retrieve created epic")?;
            Ok(serde_json::to_value(epic)?)
        }
        EpicAction::List => {
            let epics = db.list_epics()?;
            Ok(serde_json::to_value(epics)?)
        }
        EpicAction::Delete { id } => {
            db.delete_epic(id)?;
            Ok(serde_json::json!({ "deleted": id }))
        }
    }
}

fn handle_story(action: StoryAction, db: &Database) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    match action {
        StoryAction::Create { title, epic, priority, body, status } => {
            let id = db.create_story(&title, &body, epic, status, priority)?;
            let story = db.get_story(id)?;
            Ok(serde_json::to_value(story)?)
        }
        StoryAction::List { epic, status } => {
            let stories = db.list_stories(status, epic)?;
            Ok(serde_json::to_value(stories)?)
        }
        StoryAction::Get { id } => {
            let story = db.get_story(id)?;
            Ok(serde_json::to_value(story)?)
        }
        StoryAction::Update { id, title, status, priority, body, epic } => {
            if let Some(t) = title {
                db.update_story_title(id, &t)?;
            }
            if let Some(s) = status {
                db.update_story_status(id, s)?;
            }
            if let Some(p) = priority {
                db.update_story_priority(id, p)?;
            }
            if let Some(b) = body {
                db.update_story_description(id, &b)?;
            }
            if let Some(e) = epic {
                db.update_story_epic(id, Some(e))?;
            }
            let story = db.get_story(id)?;
            Ok(serde_json::to_value(story)?)
        }
        StoryAction::Delete { id } => {
            db.delete_story(id)?;
            Ok(serde_json::json!({ "deleted": id }))
        }
    }
}

fn handle_task(action: TaskAction, db: &Database) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    match action {
        TaskAction::Create { story_id, title } => {
            let id = db.create_task(story_id, &title)?;
            let task = db.get_task(id)?;
            Ok(serde_json::to_value(task)?)
        }
        TaskAction::List { story_id } => {
            let tasks = db.list_tasks(story_id)?;
            Ok(serde_json::to_value(tasks)?)
        }
        TaskAction::Toggle { id } => {
            let task = db.toggle_task(id)?;
            Ok(serde_json::to_value(task)?)
        }
        TaskAction::Delete { id } => {
            db.delete_task(id)?;
            Ok(serde_json::json!({ "deleted": id }))
        }
    }
}

fn handle_board(epic: Option<i64>, db: &Database) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let mut board: HashMap<&str, serde_json::Value> = HashMap::new();
    for status in Status::all() {
        let stories = db.list_stories_by_status(*status, epic)?;
        let key = match status {
            Status::ToDo => "todo",
            Status::InProgress => "in_progress",
            Status::InReview => "in_review",
            Status::Done => "done",
        };
        board.insert(key, serde_json::to_value(stories)?);
    }
    Ok(serde_json::to_value(board)?)
}
