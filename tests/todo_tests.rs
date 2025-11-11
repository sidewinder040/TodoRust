use todo::TodoItem;
use owo_colors::OwoColorize;

#[test]
fn test_new_and_format() {
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
fn test_format_empty() {
    let t = TodoItem::new("", "");
    let expected = format!(
        "Title: {}\nDescription: {}",
        "".bold().bright_white(),
        "".dimmed()
    );
    assert_eq!(t.format(), expected);
}
