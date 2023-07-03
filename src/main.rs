fn main() {
    let mut todo_list = TodoList::new();

    todo_list.add_todo(String::from("This is the first todo"));

    todo_list.print_todos();
}

struct TodoList {
    pub todos: Vec<TodoItem>,
}

struct TodoItem {
    id: u32,
    description: String,
    // completed: bool,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList { todos: Vec::new() }
    }

    pub fn add_todo(&mut self, description: String) {
        let id = (self.todos.len() + 1) as u32;

        let new_todo = TodoItem {
            id,
            description,
            // completed: false
        };

        self.todos.push(new_todo);
    }

    pub fn print_todos(&self) {
        for todo in &self.todos {
            println!("{},{}", todo.id, todo.description);
        }
    }


    // pub fn remove(&self) -> void {

    // }

    // pub fn clear(&self) -> void {

    // }
}