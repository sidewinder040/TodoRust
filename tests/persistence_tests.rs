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
    let loaded = TodoList::load(&path);
    assert_eq!(loaded.items.len(), 2);
    assert_eq!(loaded.items[0].title, "T1");
}
