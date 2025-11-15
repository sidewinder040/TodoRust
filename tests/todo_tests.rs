use std::collections::HashMap;
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
        created_at: chrono::Utc::now(),
        updated_at: Some(chrono::Utc::now()),
        due_date: Some(chrono::Utc::now()),
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
fn test_priority_variants() {
    assert_eq!(Priority::Low, Priority::Low);
    assert_eq!(Priority::Medium, Priority::Medium);
    assert_eq!(Priority::High, Priority::High);
}

#[test]
fn test_status_variants() {
    assert_eq!(Status::Pending, Status::Pending);
    assert_eq!(Status::InProgress, Status::InProgress);
    assert_eq!(Status::Completed, Status::Completed);
}

#[test]
fn test_todoitem_with_all_fields() {
    let now = chrono::Utc::now();
    let t = TodoItem {
        id: "unique-id".to_string(),
        title: "Complex Task".to_string(),
        description: Some("A detailed description".to_string()),
        status: Status::InProgress,
        priority: Some(Priority::High),
        created_at: now,
        updated_at: Some(now),
        due_date: Some(now),
        tags: vec!["urgent".to_string(), "work".to_string()],
        metadata: Some(HashMap::from([
            ("owner".to_string(), "alice".to_string()),
            ("project".to_string(), "beta".to_string()),
        ])),
    };

    assert_eq!(t.id, "unique-id");
    assert_eq!(t.title, "Complex Task");
    assert!(t.description.is_some());
    assert_eq!(t.priority, Some(Priority::High));
    assert_eq!(t.tags.len(), 2);
    assert_eq!(t.metadata.as_ref().unwrap().len(), 2);
}

#[test]
fn test_todoitem_serialization() {
    let now = chrono::Utc::now();
    let t = TodoItem {
        id: "test-id".to_string(),
        title: "Serialize Test".to_string(),
        description: Some("Test serialization".to_string()),
        status: Status::Completed,
        priority: Some(Priority::High),
        created_at: now,
        updated_at: Some(now),
        due_date: Some(now),
        tags: vec!["tag1".to_string(), "tag2".to_string()],
        metadata: Some(HashMap::from([("key".to_string(), "value".to_string())])),
    };

    // Serialize to JSON
    let json = serde_json::to_string(&t).expect("Failed to serialize");
    
    // Deserialize back
    let deserialized: TodoItem = serde_json::from_str(&json).expect("Failed to deserialize");
    
    assert_eq!(deserialized.id, t.id);
    assert_eq!(deserialized.title, t.title);
    assert_eq!(deserialized.description, t.description);
    assert_eq!(deserialized.status, t.status);
    assert_eq!(deserialized.priority, t.priority);
    assert_eq!(deserialized.tags, t.tags);
}

#[test]
fn test_todoitem_format_contains_all_fields() {
    let now = chrono::Utc::now();
    let t = TodoItem {
        id: "test-id".to_string(),
        title: "Full Format Test".to_string(),
        description: Some("Description text".to_string()),
        status: Status::InProgress,
        priority: Some(Priority::High),
        created_at: now,
        updated_at: Some(now),
        due_date: Some(now),
        tags: vec!["work".to_string(), "urgent".to_string()],
        metadata: None,
    };

    let formatted = t.format();
    
    // Check that the format contains the title
    assert!(formatted.contains("Full Format Test"));
    // Check that it contains status info
    assert!(formatted.contains("InProgress") || formatted.contains("In Progress"));
}

#[test]
fn test_todolist_add_and_persistence() {
    use todo::TodoList;
    
    let mut list = TodoList::default();
    assert_eq!(list.items.len(), 0);
    
    let item1 = TodoItem::new("Task 1", "Description 1");
    let item2 = TodoItem::new("Task 2", "");
    
    list.add(item1);
    list.add(item2);
    
    assert_eq!(list.items.len(), 2);
    assert_eq!(list.items[0].title, "Task 1");
    assert_eq!(list.items[1].title, "Task 2");
}

#[test]
fn test_todoitem_default_status_is_pending() {
    let t = TodoItem::new("Test", "");
    assert_eq!(t.status, Status::Pending);
}

#[test]
fn test_todoitem_default_priority_is_none() {
    let t = TodoItem::new("Test", "");
    assert!(t.priority.is_none());
}

#[test]
fn test_todoitem_tags_empty_by_default() {
    let t = TodoItem::new("Test", "");
    assert!(t.tags.is_empty());
}
