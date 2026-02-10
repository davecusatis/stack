use clap::{Parser, Subcommand};
use crate::models::{Status, Priority};

#[derive(Parser)]
#[command(name = "stack", version, about = "Terminal kanban board for personal task tracking")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    /// Manage epics
    Epic {
        #[command(subcommand)]
        action: EpicAction,
    },
    /// Manage stories
    Story {
        #[command(subcommand)]
        action: StoryAction,
    },
    /// Manage tasks within stories
    Task {
        #[command(subcommand)]
        action: TaskAction,
    },
    /// Show the full board state
    Board {
        /// Filter by epic ID
        #[arg(long)]
        epic: Option<i64>,
    },
}

#[derive(Subcommand)]
pub enum EpicAction {
    /// Create a new epic
    Create {
        #[arg(long)]
        title: String,
        #[arg(long, default_value = "")]
        description: String,
        #[arg(long, default_value = "white")]
        color: String,
    },
    /// List all epics
    List,
    /// Delete an epic by ID
    Delete {
        id: i64,
    },
}

#[derive(Subcommand)]
pub enum StoryAction {
    /// Create a new story
    Create {
        #[arg(long)]
        title: String,
        #[arg(long)]
        epic: Option<i64>,
        #[arg(long, default_value = "medium")]
        priority: Priority,
        #[arg(long, default_value = "")]
        body: String,
        #[arg(long, default_value = "todo")]
        status: Status,
    },
    /// List stories with optional filters
    List {
        #[arg(long)]
        epic: Option<i64>,
        #[arg(long)]
        status: Option<Status>,
    },
    /// Get a story by ID
    Get {
        id: i64,
    },
    /// Update a story
    Update {
        id: i64,
        #[arg(long)]
        title: Option<String>,
        #[arg(long)]
        status: Option<Status>,
        #[arg(long)]
        priority: Option<Priority>,
        #[arg(long)]
        body: Option<String>,
        #[arg(long)]
        epic: Option<i64>,
    },
    /// Delete a story by ID
    Delete {
        id: i64,
    },
}

#[derive(Subcommand)]
pub enum TaskAction {
    /// Create a new task in a story
    Create {
        story_id: i64,
        #[arg(long)]
        title: String,
    },
    /// List tasks in a story
    List {
        story_id: i64,
    },
    /// Toggle a task's done status
    Toggle {
        id: i64,
    },
    /// Delete a task by ID
    Delete {
        id: i64,
    },
}
