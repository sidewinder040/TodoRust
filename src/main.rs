use todo::{TodoItem, TodoList};

use std::env;
use std::io::{self, Write};
use owo_colors::OwoColorize;
use atty::Stream;
use directories::ProjectDirs; // added
use std::path::PathBuf;       // added

/// Choose the todo file to use and whether it was explicitly provided by the
/// user. Returns (path, overridden) where `overridden` is true when the path
/// came from a CLI argument or TODO_FILE env var.
fn choose_todo_file() -> (String, bool) {
    // Priority: first non-flag CLI arg
    for a in env::args().skip(1) {
        if !a.starts_with('-') {
            return (a, true);
        }
    }
    // Then env var
    if let Ok(envp) = env::var("TODO_FILE") {
        return (envp, true);
    }
    // fallback to platform-specific per-user file
    (default_user_todo_file().to_string_lossy().into_owned(), false)
}

// New helper: returns per-user data-file path like:
//  - Linux:  ~/.local/share/TodoRust/todos.json
//  - macOS:  ~/Library/Application Support/TodoRust/todos.json
//  - Windows: %APPDATA%\TodoRust\todos.json
fn default_user_todo_file() -> PathBuf {
    if let Some(proj) = ProjectDirs::from("com", "example", "TodoRust") {
        let dir = proj.data_local_dir(); // platform-appropriate user data directory
        if let Err(_) = std::fs::create_dir_all(dir) {
            // fall back to current dir file on failure
            return PathBuf::from("todos.json");
        }
        return dir.join("todos.json");
    }
    // final fallback
    PathBuf::from("todos.json")
}

fn main() {
    // Determine color policy: default = TTY-detection. Allow CLI override with
    // `--no-color` or `--color`. We set the NO_COLOR env var when colors are
    // disabled so library code can respect the setting.
    let mut colors = atty::is(Stream::Stdout);
    for a in env::args().skip(1) {
        if a == "--no-color" {
            colors = false;
        } else if a == "--color" {
            colors = true;
        }
    }
    if colors {
        env::remove_var("NO_COLOR");
    } else {
        env::set_var("NO_COLOR", "1");
    }

    let (todo_file, overridden) = choose_todo_file();

    // Cleanup any stale temp file left from a previous interrupted save.
    match TodoList::remove_temp_file(&todo_file) {
        Ok(true) => println!("Removed stale temp file for {}", &todo_file),
        Ok(false) => {}
        Err(e) => eprintln!("Warning: failed to clean up temp file: {}", e),
    }

    // If the user did not override the storage location, attempt to migrate a
    // local ./todos.json into the per-user location on first run.
    if !overridden {
        match TodoList::migrate_local_if_present(&todo_file) {
            Ok(true) => println!("Migrated local ./todos.json into {}", &todo_file),
            Ok(false) => {}
            Err(e) => eprintln!("Migration warning: {}", e),
        }
    }

    // Load existing todos from disk (if any), with graceful fallback to empty list
    let mut list = TodoList::load_or_empty(&todo_file);

    print_banner(&todo_file, list.items.len());

    loop {
        print_commands();

        let cmd = prompt_input("Command");
        match cmd.trim().to_lowercase().as_str() {
            "l" | "list" => {
                print_separator();
                if list.items.is_empty() {
                    println!("{}", "No todos.".italic().dimmed());
                } else {
                    for (i, t) in list.items.iter().enumerate() {
                        println!("{:>3}. {}", i + 1, t.format().bright_white());
                        if let Some(ref desc) = t.description {
                            if !desc.is_empty() {
                                // Indent description and dim it for readability
                                println!("      {}", desc.dimmed());
                            }
                        }
                    }
                }
                print_separator();
            }
            "a" | "add" => {
                // create_validated() handles all field prompts including priority, due_date, and tags
                let item = create_validated(None);
                list.add(item);
                println!("{}", "Added.".green().bold());
            }
            "r" | "remove" => {
                if list.items.is_empty() {
                    println!("No todos to remove.");
                    continue;
                }
                println!("Enter index to remove (1-{}):", list.items.len());
                if let Some(i) = read_index() {
                    if i == 0 || i > list.items.len() {
                        println!("{}", "Index out of range".red());
                    } else {
                        println!("{} {}", "Removing:".yellow(), list.items[i - 1].format().bright_white());
                        list.items.remove(i - 1);
                        println!("{}", "Removed.".green().bold());
                    }
                }
            }
            "e" | "edit" => {
                if list.items.is_empty() {
                    println!("No todos to edit.");
                    continue;
                }
                println!("Enter index to edit (1-{}):", list.items.len());
                if let Some(i) = read_index() {
                    if i == 0 || i > list.items.len() {
                        println!("{}", "Index out of range".red());
                    } else {
                        let idx = i - 1;
                        println!("{} {}", "Editing todo".yellow(), format!("{}: {}", i, list.items[idx].format()).bright_white());
                        let old = list.items[idx].clone();
                        let mut new = create_validated(Some(&old));
                        // preserve identity and timestamps
                        new.id = old.id.clone();
                        new.created_at = old.created_at;
                        new.updated_at = Some(chrono::Utc::now());
                        // carry over optional fields when left blank
                        if new.priority.is_none() {
                            new.priority = old.priority.clone();
                        }
                        if new.tags.is_empty() {
                            new.tags = old.tags.clone();
                        }

                        list.items[idx] = new;
                        println!("{}", "Updated.".green().bold());
                    }
                }
            }
            "s" | "status" => {
                if list.items.is_empty() {
                    println!("No todos to modify.");
                    continue;
                }
                println!("Enter index to change status (1-{}):", list.items.len());
                if let Some(i) = read_index() {
                    if i == 0 || i > list.items.len() {
                        println!("{}", "Index out of range".red());
                    } else {
                        let idx = i - 1;
                        let current = &list.items[idx];
                        println!("{} {} (current status: {})", "Current todo:".yellow(), current.title.bright_white(), format_status(&current.status));
                        
                        println!("\nNew status options:");
                        println!("  1 - Pending");
                        println!("  2 - InProgress");
                        println!("  3 - Completed");
                        
                        let status_choice = prompt_input("Choose status");
                        let new_status = match status_choice.trim() {
                            "1" => Some(todo::Status::Pending),
                            "2" => Some(todo::Status::InProgress),
                            "3" => Some(todo::Status::Completed),
                            _ => {
                                println!("{}", "Invalid choice".red());
                                None
                            }
                        };
                        
                        if let Some(s) = new_status {
                            list.items[idx].status = s;
                            list.items[idx].updated_at = Some(chrono::Utc::now());
                            println!("{}", "Status updated.".green().bold());
                        }
                    }
                }
            }
            "q" | "quit" => {
                match list.save(&todo_file) {
                    Ok(()) => println!("{}", format!("Saved {} todos to {}", list.items.len(), todo_file).green()),
                    Err(e) => println!("{}", format!("Failed to save todos: {}", e).red()),
                }
                break;
            }
            other if other.trim().is_empty() => continue,
            _ => println!("Unknown command: {}", cmd.trim()),
        }
    }
}

fn print_banner(todo_file: &str, count: usize) {
    print_separator();
    let title = format!(" TodoRust — {} todos (file: {}) ", count, todo_file);
    println!("{}", title.bold().bright_blue());
    print_separator();
}

fn print_separator() {
    println!("{}", "-".repeat(50).dimmed());
}

fn format_status(status: &todo::Status) -> String {
    match status {
        todo::Status::Pending => "Pending".yellow().to_string(),
        todo::Status::InProgress => "InProgress".cyan().to_string(),
        todo::Status::Completed => "Completed".green().to_string(),
    }
}

fn print_commands() {
    println!("{}", "Commands:".bold().underline());
    println!("  {} - {}", format!("{:<8}", "l, list").cyan(), "List todos".dimmed());
    println!("  {} - {}", format!("{:<8}", "a, add").cyan(), "Add a todo".dimmed());
    println!("  {} - {}", format!("{:<8}", "e, edit").cyan(), "Edit by index".dimmed());
    println!("  {} - {}", format!("{:<8}", "s, status").cyan(), "Change status".dimmed());
    println!("  {} - {}", format!("{:<8}", "r, remove").cyan(), "Remove by index".dimmed());
    println!("  {} - {}", format!("{:<8}", "q, quit").cyan(), "Save and quit".dimmed());
}

fn prompt_input(prompt: &str) -> String {
    print!("{}: ", prompt.cyan().bold());
    io::stdout().flush().ok();
    let mut s = String::new();
    if io::stdin().read_line(&mut s).is_err() {
        println!("{}", "Failed to read input".red());
        return String::new();
    }
    s.trim().to_string()
}

fn read_index() -> Option<usize> {
    let s = prompt_input(">");
    if s.is_empty() {
        println!("{}", "No input provided".yellow());
        return None;
    }
    match s.trim().parse::<usize>() {
        Ok(n) => Some(n),
        Err(_) => {
            println!("{}", "Invalid number".red());
            None
        }
    }
}
fn prompt_default(prompt: &str, default: Option<&str>) -> String {
    match default {
        Some(d) if !d.is_empty() => {
            print!("{} [{}]: ", prompt.cyan().bold(), d.dimmed());
        }
        _ => {
            print!("{}: ", prompt.cyan().bold());
        }
    }
    io::stdout().flush().ok();
    let mut s = String::new();
    if io::stdin().read_line(&mut s).is_err() {
        println!("{}", "Failed to read input".red());
        return String::new();
    }
    let s = s.trim().to_string();
    s
}

fn create_validated(existing: Option<&TodoItem>) -> TodoItem {
    // Title (required) - on edit, pressing enter keeps the existing title
    let title = loop {
        let def = existing.map(|e| e.title.as_str());
        let input = prompt_default("Title (required)", def);
        if !input.is_empty() {
            break input;
        }
        if let Some(e) = existing {
            if !e.title.is_empty() {
                break e.title.clone();
            }
        }
        println!("{}", "Title cannot be empty. Please try again.".red());
    };

    // Description (optional)
    let def_desc = existing.and_then(|e| e.description.as_deref());
    let desc_in = prompt_default("Description", def_desc);
    let description = if desc_in.is_empty() {
        def_desc.map(|s| s.to_string())
    } else {
        Some(desc_in)
    };

    // Priority
    let def_pri = existing.and_then(|e| e.priority.as_ref()).map(|p| match p {
        todo::Priority::Low => "low",
        todo::Priority::Medium => "med",
        todo::Priority::High => "high",
    });
    let pri_in = prompt_default("Priority (low/med/high) [blank for none]", def_pri);
    let priority = if pri_in.is_empty() {
        existing.and_then(|e| e.priority.clone())
    } else {
        match pri_in.to_lowercase().as_str() {
            "low" => Some(todo::Priority::Low),
            "high" => Some(todo::Priority::High),
            _ => Some(todo::Priority::Medium),
        }
    };

    // Due date
    let def_due = existing.and_then(|e| e.due_date.as_ref()).map(|d| d.format("%Y-%m-%d").to_string());
    let due_in = prompt_default("Due date (YYYY-MM-DD) [blank for none]", def_due.as_deref());
    let due_date = if due_in.is_empty() {
        existing.and_then(|e| e.due_date.clone())
    } else {
        match chrono::NaiveDate::parse_from_str(&due_in, "%Y-%m-%d") {
            Ok(nd) => nd.and_hms_opt(0, 0, 0).map(|naive| chrono::DateTime::from_naive_utc_and_offset(naive, chrono::Utc)),
            Err(_) => {
                println!("{}", "Invalid date format; ignoring due date".yellow());
                existing.and_then(|e| e.due_date.clone())
            }
        }
    };

    // Tags
    let def_tags = if let Some(e) = existing { if !e.tags.is_empty() { Some(e.tags.join(",")) } else { None } } else { None };
    let tags_in = prompt_default("Tags (comma separated) [blank for none]", def_tags.as_deref());
    let tags = if tags_in.is_empty() {
        existing.map(|e| e.tags.clone()).unwrap_or_default()
    } else {
        tags_in.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect()
    };

    // Build the TodoItem, carrying over id/created_at/metadata when editing
    if let Some(e) = existing {
        TodoItem {
            id: e.id.clone(),
            title,
            description,
            status: e.status.clone(),
            priority,
            created_at: e.created_at,
            updated_at: Some(chrono::Utc::now()),
            due_date,
            tags,
            metadata: e.metadata.clone(),
        }
    } else {
        // Use the public constructor to ensure fields like id/created_at are set
        let mut item = TodoItem::new(&title, description.as_deref().unwrap_or(""));
        item.priority = priority;
        item.due_date = due_date;
        item.tags = tags;
        item
    }
}

