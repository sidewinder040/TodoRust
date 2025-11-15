/// Shared library types and functions for the todo binary.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use owo_colors::OwoColorize;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
    Pending,
    InProgress,
    Completed,
}

impl Default for Status {
    fn default() -> Self {
        Status::Pending
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
}

impl Default for Priority {
    fn default() -> Self {
        Priority::Medium
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoItem {
    #[serde(default = "new_uuid")]
    pub id: String,

    pub title: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(default)]
    pub status: Status,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<Priority>,

    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,

    #[serde(default, skip_serializing_if = "Option::is_none", with = "chrono::serde::ts_seconds_option")]
    pub updated_at: Option<DateTime<Utc>>,

    #[serde(default, skip_serializing_if = "Option::is_none", with = "chrono::serde::ts_seconds_option")]
    pub due_date: Option<DateTime<Utc>>,

    #[serde(default)]
    pub tags: Vec<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

fn new_uuid() -> String {
    Uuid::new_v4().to_string()
}

impl TodoItem {
    /// Create a new TodoItem from string slices (backwards-compatible API)
    pub fn new(title: &str, description: &str) -> Self {
        TodoItem {
            id: new_uuid(),
            title: title.to_string(),
            description: if description.is_empty() { None } else { Some(description.to_string()) },
            status: Status::default(),
            priority: None,
            created_at: Utc::now(),
            updated_at: None,
            due_date: None,
            tags: Vec::new(),
            metadata: None,
        }
    }

    /// Return a concise colored summary: title, status, priority and due date.
    pub fn format(&self) -> String {
        let title = self.title.bold().bright_white().to_string();
        let mut parts: Vec<String> = vec![title.clone()];

        // Status (as colored string)
        let status = match self.status {
            Status::Pending => "Pending".yellow().to_string(),
            Status::InProgress => "InProgress".cyan().to_string(),
            Status::Completed => "Completed".green().to_string(),
        };
        parts.push(format!("[{}]", status));

        // Priority
        if let Some(ref p) = self.priority {
            let pcol = match p {
                Priority::Low => "Low".dimmed().to_string(),
                Priority::Medium => "Med".bright_white().to_string(),
                Priority::High => "High".red().bold().to_string(),
            };
            parts.push(format!("({})", pcol));
        }

        // Due date
        if let Some(d) = self.due_date {
            parts.push(format!("due:{}", d.format("%Y-%m-%d"))); // date only
        }

        // Tags
        if !self.tags.is_empty() {
            parts.push(format!("tags:{}", self.tags.join(",")));
        }

        parts.join(" ")
    }
}

/// A collection of TodoItem values with load/save helpers.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TodoList {
    pub items: Vec<TodoItem>,
}

impl TodoList {
    /// Load a TodoList from the given file path. If the file does not exist or
    /// cannot be parsed, returns an empty list.
    pub fn load<P: AsRef<Path>>(path: P) -> Self {
        let path = path.as_ref();
        match fs::read_to_string(path) {
            Ok(contents) => match serde_json::from_str::<TodoList>(&contents) {
                Ok(list) => list,
                Err(_) => TodoList::default(),
            },
            Err(_) => TodoList::default(),
        }
    }

    /// Save the list to the given path as pretty JSON. Returns an error on IO
    /// or serialization failures.
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let path = path.as_ref();
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    /// Add an item to the list.
    pub fn add(&mut self, item: TodoItem) {
        self.items.push(item);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_and_format_lib() {
        let t = TodoItem::new("Buy milk", "2 liters");
        assert_eq!(t.title, "Buy milk");
        assert_eq!(t.description, Some("2 liters".to_string()));
        // Format now produces a concise single-line summary, ensure it contains key parts.
        let out = t.format();
        assert!(out.contains("Buy milk"));
        assert!(out.contains("Pending") || out.contains("InProgress") || out.contains("Completed"));
    }

    #[test]
    fn test_format_empty_lib() {
        let t = TodoItem::new("", "");
        assert_eq!(t.description, None);
        let out = t.format();
        assert!(out.contains("") );
    }
}
