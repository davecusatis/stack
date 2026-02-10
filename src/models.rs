use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Epic {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub color: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
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
}
