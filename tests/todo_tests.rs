use todo::TodoItem;

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
