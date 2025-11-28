use std::fs;
// std::path::Path not needed here
use tempfile::tempdir;
use todo::{TodoItem, TodoList};

#[test]
fn migrate_local_todos_into_target() {
    // Create a temporary directory and make it the current working directory
    let td = tempdir().expect("create tempdir");
    let cwd = std::env::current_dir().expect("cwd");
    std::env::set_current_dir(td.path()).expect("chdir");

    // Create a local ./todos.json
    let mut list = TodoList::default();
    list.add(TodoItem::new("Local T1", "desc"));
    let local_path = td.path().join("todos.json");
    fs::write(&local_path, serde_json::to_string_pretty(&list).unwrap()).expect("write local todos");

    // Choose a target path (under a subdir) that does not exist yet
    let target_dir = td.path().join("user_data");
    let target_file = target_dir.join("todos.json");

    // Ensure target doesn't exist
    assert!(!target_file.exists());

    // Run migration
    let migrated = TodoList::migrate_local_if_present(&target_file).expect("migration call");
    assert!(migrated, "migration should have occurred");

    // Local file should be gone, target file should exist
    assert!(!local_path.exists(), "local todos.json should have been moved");
    assert!(target_file.exists(), "target todos.json should exist after migration");

    // Loading the migrated file should yield our item
    let loaded = TodoList::load(&target_file).expect("load migrated");
    assert_eq!(loaded.items.len(), 1);
    assert_eq!(loaded.items[0].title, "Local T1");

    // restore cwd
    std::env::set_current_dir(cwd).expect("restore cwd");
}

#[test]
fn atomic_save_creates_file_and_removes_temp() {
    let td = tempdir().expect("create tempdir");
    let target_file = td.path().join("todos.json");

    let mut list = TodoList::default();
    list.add(TodoItem::new("A1", "D1"));

    // Save should succeed
    list.save(&target_file).expect("save");

    // File should exist and be valid JSON
    assert!(target_file.exists());
    let read = fs::read_to_string(&target_file).expect("read saved");
    let parsed: TodoList = serde_json::from_str(&read).expect("parse saved");
    assert_eq!(parsed.items.len(), 1);

    // tmp file name should not exist (hidden temp name used by save)
    let tmp_name = format!(".{}.tmp", target_file.file_name().unwrap().to_string_lossy());
    let tmp_path = target_file.with_file_name(tmp_name);
    assert!(!tmp_path.exists(), "temp file should be removed after successful save");
}
