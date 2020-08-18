struct TodoItem {
    title: String,
    description: String,
}

fn main() {
    let todo1 = TodoItem {
        title: String::from("Go to shops"),
        description:String::from("I need to buy milk")
    };
    display(todo1);
}

fn display(todo: TodoItem) {
   println!("Title: {}", todo.title);
   println!("Description: {}", todo.description);
}
