/// Shared library types and functions for the todo binary.
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use owo_colors::OwoColorize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TodoItem {
    pub title: String,
    pub description: String,
}

impl TodoItem {
    /// Create a new TodoItem from string slices.
    pub fn new(title: &str, description: &str) -> Self {
        TodoItem {
            title: title.to_string(),
            description: description.to_string(),
        }
    }

    /// Return a formatted (colored) representation used by `display`.
    pub fn format(&self) -> String {
        // Color the title prominently and dim the description for readability.
        format!(
            "Title: {}\nDescription: {}",
            self.title.bold().bright_white(),
            self.description.dimmed()
        )
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
        assert_eq!(t.description, "2 liters");
        let expected = format!(
            "Title: {}\nDescription: {}",
            "Buy milk".bold().bright_white(),
            "2 liters".dimmed()
        );
        assert_eq!(t.format(), expected);
    }

    #[test]
    fn test_format_empty_lib() {
        let t = TodoItem::new("", "");
        let expected = format!(
            "Title: {}\nDescription: {}",
            "".bold().bright_white(),
            "".dimmed()
        );
        assert_eq!(t.format(), expected);
    }
}
