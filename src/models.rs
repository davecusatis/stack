use std::fmt;
use std::str::FromStr;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    ToDo,
    InProgress,
    InReview,
    Done,
}

impl Status {
    pub fn as_str(&self) -> &'static str {
        match self {
            Status::ToDo => "To Do",
            Status::InProgress => "In Progress",
            Status::InReview => "In Review",
            Status::Done => "Done",
        }
    }

    pub fn all() -> &'static [Status] {
        &[Status::ToDo, Status::InProgress, Status::InReview, Status::Done]
    }

    pub fn next(&self) -> Option<Status> {
        match self {
            Status::ToDo => Some(Status::InProgress),
            Status::InProgress => Some(Status::InReview),
            Status::InReview => Some(Status::Done),
            Status::Done => None,
        }
    }

    pub fn prev(&self) -> Option<Status> {
        match self {
            Status::ToDo => None,
            Status::InProgress => Some(Status::ToDo),
            Status::InReview => Some(Status::InProgress),
            Status::Done => Some(Status::InReview),
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for Status {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "todo" | "to_do" | "to-do" => Ok(Status::ToDo),
            "in-progress" | "in_progress" => Ok(Status::InProgress),
            "in-review" | "in_review" => Ok(Status::InReview),
            "done" => Ok(Status::Done),
            _ => Err(format!("unknown status: '{}' (expected: todo, in-progress, in-review, done)", s)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

impl Priority {
    pub fn as_str(&self) -> &'static str {
        match self {
            Priority::Low => "Low",
            Priority::Medium => "Medium",
            Priority::High => "High",
            Priority::Critical => "Critical",
        }
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for Priority {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "low" => Ok(Priority::Low),
            "medium" => Ok(Priority::Medium),
            "high" => Ok(Priority::High),
            "critical" => Ok(Priority::Critical),
            _ => Err(format!("unknown priority: '{}' (expected: low, medium, high, critical)", s)),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize)]
pub struct Epic {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub color: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize)]
pub struct Story {
    pub id: i64,
    pub epic_id: Option<i64>,
    pub title: String,
    pub description: String,
    pub status: Status,
    pub priority: Priority,
    pub created_at: String,
    pub updated_at: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize)]
pub struct Task {
    pub id: i64,
    pub story_id: i64,
    pub title: String,
    pub done: bool,
    pub sort_order: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_display_and_variants() {
        assert_eq!(Status::ToDo.as_str(), "To Do");
        assert_eq!(Status::InProgress.as_str(), "In Progress");
        assert_eq!(Status::InReview.as_str(), "In Review");
        assert_eq!(Status::Done.as_str(), "Done");
    }

    #[test]
    fn status_all_returns_four_columns() {
        assert_eq!(Status::all().len(), 4);
    }

    #[test]
    fn priority_ordering() {
        assert!(Priority::Critical > Priority::High);
        assert!(Priority::High > Priority::Medium);
        assert!(Priority::Medium > Priority::Low);
    }

    #[test]
    fn priority_display() {
        assert_eq!(Priority::Low.as_str(), "Low");
        assert_eq!(Priority::Critical.as_str(), "Critical");
    }

    #[test]
    fn status_from_str() {
        assert_eq!("todo".parse::<Status>().unwrap(), Status::ToDo);
        assert_eq!("in-progress".parse::<Status>().unwrap(), Status::InProgress);
        assert_eq!("in-review".parse::<Status>().unwrap(), Status::InReview);
        assert_eq!("done".parse::<Status>().unwrap(), Status::Done);
        assert!("invalid".parse::<Status>().is_err());
    }

    #[test]
    fn priority_from_str() {
        assert_eq!("low".parse::<Priority>().unwrap(), Priority::Low);
        assert_eq!("medium".parse::<Priority>().unwrap(), Priority::Medium);
        assert_eq!("high".parse::<Priority>().unwrap(), Priority::High);
        assert_eq!("critical".parse::<Priority>().unwrap(), Priority::Critical);
        assert!("invalid".parse::<Priority>().is_err());
    }
}
