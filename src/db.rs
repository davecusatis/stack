use rusqlite::{Connection, Result};
use crate::models::{Epic, Story, Status, Priority};

pub struct Database {
    conn: Connection,
}

fn status_to_db(s: &Status) -> &'static str {
    match s {
        Status::ToDo => "todo",
        Status::InProgress => "in_progress",
        Status::InReview => "in_review",
        Status::Done => "done",
    }
}

fn status_from_db(s: &str) -> Status {
    match s {
        "in_progress" => Status::InProgress,
        "in_review" => Status::InReview,
        "done" => Status::Done,
        _ => Status::ToDo,
    }
}

fn priority_to_db(p: &Priority) -> &'static str {
    match p {
        Priority::Low => "low",
        Priority::Medium => "medium",
        Priority::High => "high",
        Priority::Critical => "critical",
    }
}

fn priority_from_db(s: &str) -> Priority {
    match s {
        "low" => Priority::Low,
        "high" => Priority::High,
        "critical" => Priority::Critical,
        _ => Priority::Medium,
    }
}

impl Database {
    pub fn open(path: &std::path::Path) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;
        Ok(Database { conn })
    }

    pub fn migrate(&mut self) -> Result<()> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS epics (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                description TEXT NOT NULL DEFAULT '',
                color TEXT NOT NULL DEFAULT 'white'
            );

            CREATE TABLE IF NOT EXISTS stories (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                epic_id INTEGER REFERENCES epics(id) ON DELETE SET NULL,
                title TEXT NOT NULL,
                description TEXT NOT NULL DEFAULT '',
                status TEXT NOT NULL DEFAULT 'todo',
                priority TEXT NOT NULL DEFAULT 'medium',
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                updated_at TEXT NOT NULL DEFAULT (datetime('now'))
            );

            CREATE TABLE IF NOT EXISTS tasks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                story_id INTEGER NOT NULL REFERENCES stories(id) ON DELETE CASCADE,
                title TEXT NOT NULL,
                done INTEGER NOT NULL DEFAULT 0,
                sort_order INTEGER NOT NULL DEFAULT 0
            );
            -- Legacy table kept for migration compat; not used by app"
        )?;
        Ok(())
    }

    // --- Epic CRUD ---

    #[allow(dead_code)]
    pub fn create_epic(&self, title: &str, description: &str, color: &str) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO epics (title, description, color) VALUES (?1, ?2, ?3)",
            rusqlite::params![title, description, color],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn list_epics(&self) -> Result<Vec<Epic>> {
        let mut stmt = self.conn.prepare("SELECT id, title, description, color FROM epics ORDER BY id")?;
        let epics = stmt.query_map([], |row| {
            Ok(Epic {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                color: row.get(3)?,
            })
        })?.collect::<Result<Vec<_>>>()?;
        Ok(epics)
    }

    #[allow(dead_code)]
    pub fn delete_epic(&self, id: i64) -> Result<()> {
        self.conn.execute("DELETE FROM epics WHERE id = ?1", [id])?;
        Ok(())
    }

    // --- Story CRUD ---

    pub fn create_story(&self, title: &str, description: &str, epic_id: Option<i64>, status: Status, priority: Priority) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO stories (title, description, epic_id, status, priority) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![title, description, epic_id, status_to_db(&status), priority_to_db(&priority)],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn list_stories_by_status(&self, status: Status, epic_id: Option<i64>) -> Result<Vec<Story>> {
        let (sql, params): (&str, Vec<Box<dyn rusqlite::types::ToSql>>) = match epic_id {
            Some(eid) => (
                "SELECT id, epic_id, title, description, status, priority, created_at, updated_at FROM stories WHERE status = ?1 AND epic_id = ?2 ORDER BY id",
                vec![Box::new(status_to_db(&status).to_string()), Box::new(eid)],
            ),
            None => (
                "SELECT id, epic_id, title, description, status, priority, created_at, updated_at FROM stories WHERE status = ?1 ORDER BY id",
                vec![Box::new(status_to_db(&status).to_string())],
            ),
        };
        let mut stmt = self.conn.prepare(sql)?;
        let stories = stmt.query_map(rusqlite::params_from_iter(params.iter()), |row| {
            let status_str: String = row.get(4)?;
            let priority_str: String = row.get(5)?;
            Ok(Story {
                id: row.get(0)?,
                epic_id: row.get(1)?,
                title: row.get(2)?,
                description: row.get(3)?,
                status: status_from_db(&status_str),
                priority: priority_from_db(&priority_str),
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        })?.collect::<Result<Vec<_>>>()?;
        Ok(stories)
    }

    pub fn update_story_status(&self, id: i64, status: Status) -> Result<()> {
        self.conn.execute(
            "UPDATE stories SET status = ?1, updated_at = datetime('now') WHERE id = ?2",
            rusqlite::params![status_to_db(&status), id],
        )?;
        Ok(())
    }

    pub fn update_story_title(&self, id: i64, title: &str) -> Result<()> {
        self.conn.execute(
            "UPDATE stories SET title = ?1, updated_at = datetime('now') WHERE id = ?2",
            rusqlite::params![title, id],
        )?;
        Ok(())
    }

    pub fn get_story(&self, id: i64) -> Result<Story> {
        self.conn.query_row(
            "SELECT id, epic_id, title, description, status, priority, created_at, updated_at FROM stories WHERE id = ?1",
            [id],
            |row| {
                let status_str: String = row.get(4)?;
                let priority_str: String = row.get(5)?;
                Ok(Story {
                    id: row.get(0)?,
                    epic_id: row.get(1)?,
                    title: row.get(2)?,
                    description: row.get(3)?,
                    status: status_from_db(&status_str),
                    priority: priority_from_db(&priority_str),
                    created_at: row.get(6)?,
                    updated_at: row.get(7)?,
                })
            },
        )
    }

    pub fn delete_story(&self, id: i64) -> Result<()> {
        self.conn.execute("DELETE FROM stories WHERE id = ?1", [id])?;
        Ok(())
    }

    pub fn update_story_description(&self, id: i64, description: &str) -> Result<()> {
        self.conn.execute(
            "UPDATE stories SET description = ?1, updated_at = datetime('now') WHERE id = ?2",
            rusqlite::params![description, id],
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    fn test_db() -> Database {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch("PRAGMA foreign_keys=ON;").unwrap();
        let mut db = Database { conn };
        db.migrate().unwrap();
        db
    }

    #[test]
    fn migrate_creates_tables() {
        let db = test_db();
        let count: i32 = db.conn.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name IN ('epics', 'stories', 'tasks')",
            [],
            |row| row.get(0),
        ).unwrap();
        assert_eq!(count, 3);
    }

    #[test]
    fn migrate_is_idempotent() {
        let mut db = test_db();
        db.migrate().unwrap();
    }

    // --- Epic tests ---

    #[test]
    fn create_and_list_epics() {
        let db = test_db();
        let id = db.create_epic("Auth System", "Build login/signup", "blue").unwrap();
        assert!(id > 0);

        let epics = db.list_epics().unwrap();
        assert_eq!(epics.len(), 1);
        assert_eq!(epics[0].title, "Auth System");
        assert_eq!(epics[0].color, "blue");
    }

    #[test]
    fn delete_epic() {
        let db = test_db();
        let id = db.create_epic("Temp", "", "white").unwrap();
        db.delete_epic(id).unwrap();
        let epics = db.list_epics().unwrap();
        assert_eq!(epics.len(), 0);
    }

    // --- Story tests ---

    #[test]
    fn create_and_list_stories_by_status() {
        let db = test_db();
        let eid = db.create_epic("E1", "", "white").unwrap();
        db.create_story("Story A", "", Some(eid), Status::ToDo, Priority::High).unwrap();
        db.create_story("Story B", "", Some(eid), Status::InProgress, Priority::Low).unwrap();
        db.create_story("Story C", "", None, Status::ToDo, Priority::Medium).unwrap();

        let todo = db.list_stories_by_status(Status::ToDo, None).unwrap();
        assert_eq!(todo.len(), 2);

        let filtered = db.list_stories_by_status(Status::ToDo, Some(eid)).unwrap();
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].title, "Story A");
    }

    #[test]
    fn update_story_status() {
        let db = test_db();
        let sid = db.create_story("S1", "", None, Status::ToDo, Priority::Medium).unwrap();
        db.update_story_status(sid, Status::InProgress).unwrap();

        let stories = db.list_stories_by_status(Status::InProgress, None).unwrap();
        assert_eq!(stories.len(), 1);
        assert_eq!(stories[0].id, sid);
    }

    #[test]
    fn delete_story() {
        let db = test_db();
        let sid = db.create_story("S1", "", None, Status::ToDo, Priority::Medium).unwrap();
        db.delete_story(sid).unwrap();
        let stories = db.list_stories_by_status(Status::ToDo, None).unwrap();
        assert_eq!(stories.len(), 0);
    }

    #[test]
    fn update_story_description() {
        let db = test_db();
        let sid = db.create_story("S1", "", None, Status::ToDo, Priority::Medium).unwrap();
        db.update_story_description(sid, "# Heading\n\nSome **markdown** body").unwrap();
        let story = db.get_story(sid).unwrap();
        assert_eq!(story.description, "# Heading\n\nSome **markdown** body");
    }
}
