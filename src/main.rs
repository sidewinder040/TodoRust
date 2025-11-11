use todo::{TodoItem, TodoList};

use std::env;
use std::io::{self, Write};
use owo_colors::OwoColorize;

fn choose_todo_file() -> String {
    // Priority: first CLI arg, then TODO_FILE env var, then default
    if let Some(arg1) = env::args().nth(1) {
        return arg1;
    }
    if let Ok(envp) = env::var("TODO_FILE") {
        return envp;
    }
    "todos.json".to_string()
}

fn main() {
    let todo_file = choose_todo_file();

    // Load existing todos from disk (if any)
    let mut list = TodoList::load(&todo_file);

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
                    }
                }
                print_separator();
            }
            "a" | "add" => {
                let item = create_validated();
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
                        let new = create_validated();
                        list.items[idx] = new;
                        println!("{}", "Updated.".green().bold());
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

fn print_commands() {
    println!("{}", "Commands:".bold().underline());
    println!("  {} - {}", format!("{:<8}", "l, list").cyan(), "List todos".dimmed());
    println!("  {} - {}", format!("{:<8}", "a, add").cyan(), "Add a todo".dimmed());
    println!("  {} - {}", format!("{:<8}", "r, remove").cyan(), "Remove by index".dimmed());
    println!("  {} - {}", format!("{:<8}", "e, edit").cyan(), "Edit by index".dimmed());
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

fn create_validated() -> TodoItem {
    loop {
        let title = prompt_input("Title (required)");
        if title.is_empty() {
            println!("{}", "Title cannot be empty. Please try again.".red());
            continue;
        }
        let description = prompt_input("Description");
        return TodoItem::new(&title, &description);
    }
}

