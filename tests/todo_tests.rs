use todo::TodoItem;

#[test]
fn test_new_and_format() {
    let t = TodoItem::new("Buy milk", "2 liters");
    assert_eq!(t.title, "Buy milk");
    assert_eq!(t.description, "2 liters");
    assert_eq!(t.format(), "Title: Buy milk\nDescription: 2 liters");
}

#[test]
fn test_format_empty() {
    let t = TodoItem::new("", "");
    assert_eq!(t.format(), "Title: \nDescription: ");
}
