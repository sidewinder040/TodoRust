use std::fs;
use tempfile::NamedTempFile;
use todo::{TodoItem, TodoList};

#[test]
fn save_and_load_roundtrip() {
    let tmp = NamedTempFile::new().expect("create temp file");
    let path = tmp.path().to_owned();

    let mut list = TodoList::default();
    list.add(TodoItem::new("T1", "D1"));
    list.add(TodoItem::new("T2", "D2"));

    // Save to temp path
    list.save(&path).expect("save should succeed");

    // Read raw file to ensure it was written
    let contents = fs::read_to_string(&path).expect("read back");
    assert!(contents.contains("T1"));

    // Load using API
    let loaded = TodoList::load(&path).expect("load should succeed");
    assert_eq!(loaded.items.len(), 2);
    assert_eq!(loaded.items[0].title, "T1");
}

#[test]
fn load_nonexistent_file_returns_error() {
    let result = TodoList::load("/nonexistent/path/todos.json");
    assert!(result.is_err());
}

#[test]
fn load_invalid_json_returns_error() {
    let tmp = NamedTempFile::new().expect("create temp file");
    let path = tmp.path().to_owned();
    
    // Write invalid JSON
    fs::write(&path, "{ invalid json").expect("write invalid json");
    
    let result = TodoList::load(&path);
    assert!(result.is_err());
}

#[test]
fn load_or_empty_returns_empty_list_on_error() {
    let list = TodoList::load_or_empty("/nonexistent/path/todos.json");
    assert_eq!(list.items.len(), 0);
}
