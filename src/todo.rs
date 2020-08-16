pub struct TodoItem {
    pub Title: String,
    pub ID: u32,
}

impl TodoItem {
    pub fn new() -> TodoItem {
        TodoItem {
            Title: String::new(),
            ID: u32::new(),
        }
    }
}

