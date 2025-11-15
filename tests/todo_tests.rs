use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::process::Command;
use todo::{Priority, Status, TodoItem};

#[test]
fn test_new_and_format() {
    let t = TodoItem::new("Buy milk", "2 liters");
    assert_eq!(t.title, "Buy milk");
    assert_eq!(t.description, Some("2 liters".to_string()));
    let out = t.format();
    assert!(out.contains("Buy milk"));
    assert!(out.contains("Pending") || out.contains("InProgress") || out.contains("Completed"));
}

#[test]
fn test_format_empty() {
    let t = TodoItem::new("", "");
    assert_eq!(t.description, None);
    let out = t.format();
    // must return a non-empty string (title may be empty)
    assert!(!out.is_empty());
}

#[test]
fn test_todoitem_fields() {
    let t = TodoItem {
        id: "test-id".to_string(),
        title: "Test Title".to_string(),
        description: Some("Test Description".to_string()),
        status: Status::InProgress,
        priority: Some(Priority::High),
        created_at: Utc::now(),
        updated_at: Some(Utc::now()),
        due_date: Some(Utc::now()),
        tags: vec!["tag1".to_string(), "tag2".to_string()],
        metadata: Some(HashMap::from([("key".to_string(), "value".to_string())])),
    };

    assert_eq!(t.title, "Test Title");
    assert_eq!(t.description, Some("Test Description".to_string()));
    assert_eq!(t.status, Status::InProgress);
    assert_eq!(t.priority, Some(Priority::High));
    assert!(t.tags.contains(&"tag1".to_string()));
    assert!(t.metadata.as_ref().unwrap().contains_key("key"));
}

#[test]
fn test_cli_add_command() {
    let output = Command::new("cargo")
        .args(["run", "--", "--no-color", "a"])
        .output()
        .expect("Failed to execute process");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Command: a"));
    assert!(stdout.contains("Title"));
}

#[test]
fn test_cli_list_command() {
    let output = Command::new("cargo")
        .args(["run", "--", "--no-color", "l"])
        .output()
        .expect("Failed to execute process");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Commands:"));
    assert!(stdout.contains("List todos"));
}
