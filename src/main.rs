use todo::{TodoItem, TodoList};

use std::env;

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

    println!("Loaded {} todos from {}.", list.items.len(), todo_file);

    loop {
        println!("\nCommands: (l)ist, (a)dd, (r)emove, (e)dit, (q)uit");
        print!("> ");
        use std::io::{self, Write};
        io::stdout().flush().ok();

        let mut cmd = String::new();
        if io::stdin().read_line(&mut cmd).is_err() {
            println!("Failed to read input");
            continue;
        }
        match cmd.trim().to_lowercase().as_str() {
            "l" | "list" => {
                if list.items.is_empty() {
                    println!("No todos.");
                } else {
                    for (i, t) in list.items.iter().enumerate() {
                        println!("{}: {}", i + 1, t.format());
                    }
                }
            }
            "a" | "add" => {
                let item = create_validated();
                list.add(item);
                println!("Added.");
            }
            "r" | "remove" => {
                if list.items.is_empty() {
                    println!("No todos to remove.");
                    continue;
                }
                println!("Enter index to remove (1-{}):", list.items.len());
                let idx = read_index();
                if let Some(i) = idx {
                    if i == 0 || i > list.items.len() {
                        println!("Index out of range");
                    } else {
                        list.items.remove(i - 1);
                        println!("Removed.");
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
                        println!("Index out of range");
                    } else {
                        let idx = i - 1;
                        println!("Editing todo {}: {}", i, list.items[idx].format());
                        let new = create_validated();
                        list.items[idx] = new;
                        println!("Updated.");
                    }
                }
            }
            "q" | "quit" => {
                match list.save(&todo_file) {
                    Ok(()) => println!("Saved {} todos to {}", list.items.len(), todo_file),
                    Err(e) => println!("Failed to save todos: {}", e),
                }
                break;
            }
            other if other.trim().is_empty() => continue,
            _ => println!("Unknown command: {}", cmd.trim()),
        }
    }
}

fn read_index() -> Option<usize> {
    use std::io::{self, Write};
    print!("> ");
    io::stdout().flush().ok();
    let mut s = String::new();
    if io::stdin().read_line(&mut s).is_err() {
        println!("Failed to read index");
        return None;
    }
    match s.trim().parse::<usize>() {
        Ok(n) => Some(n),
        Err(_) => {
            println!("Invalid number");
            None
        }
    }
}

fn create_validated() -> TodoItem {
    use std::io::{self, Write};

    loop {
        let mut title = String::new();
        let mut description = String::new();

        print!("Title (required): ");
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin().read_line(&mut title).expect("Failed to read title");
        let title = title.trim().to_string();
        if title.is_empty() {
            println!("Title cannot be empty. Please try again.");
            continue;
        }

        print!("Description: ");
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin().read_line(&mut description).expect("Failed to read description");
        let description = description.trim().to_string();

        return TodoItem::new(&title, &description);
    }
}

