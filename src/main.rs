use todo::{TodoItem, TodoList};

const TODO_FILE: &str = "todos.json";

fn main() {
    // Load existing todos from disk (if any)
    let mut list = TodoList::load(TODO_FILE);

    println!("Loaded {} todos.", list.items.len());

    loop {
        println!("\nCommands: (l)ist, (a)dd, (q)uit");
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
                let item = create();
                list.add(item);
                println!("Added.");
            }
            "q" | "quit" => {
                match list.save(TODO_FILE) {
                    Ok(()) => println!("Saved {} todos to {}", list.items.len(), TODO_FILE),
                    Err(e) => println!("Failed to save todos: {}", e),
                }
                break;
            }
            other if other.trim().is_empty() => continue,
            _ => println!("Unknown command: {}", cmd.trim()),
        }
    }
}

fn create() -> TodoItem {
    use std::io::{self, Write};

    let mut title = String::new();
    let mut description = String::new();

    print!("Title: ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut title)
        .expect("Failed to read title");
    let title = title.trim().to_string();

    print!("Description: ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut description)
        .expect("Failed to read description");
    let description = description.trim().to_string();

    TodoItem::new(&title, &description)
}

