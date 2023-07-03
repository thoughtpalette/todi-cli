extern crate colorful;
use std::io;
use colorful::Color;
use colorful::Colorful;

fn main() {
    let mut todo_list = TodoList::new();
    

    println!("{}", "Please start by adding a todo:".gradient(Color::White));
    println!("{}", "------------------------------".gradient(Color::White));
    todo_list.add_todo();

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
        TodoList { 
            todos: Vec::new()
        }
    }

    pub fn show_user_prompt(&mut self) {
        println!("{}","What would you like to do next?".gradient(Color::White));
        println!("Press to run the following commands:\n1. Show my Todo List.\n2. Add a Todo Item.\n3. Remove a Todo Item.\n4. Clear all Todo Items.");

        let mut command = String::new();

        io::stdin().read_line(&mut command).expect("Failed to read line.");

        let command_number: u32 = command.trim().parse().expect("Invalid input.");
        
        match command_number {
            1 => self.print_todos(),
            2 => self.add_todo(),
            3 => self.remove_todo(),
            4 => self.clear(),
            _ => println!("Invalid selection, please select a command from the list.")
        }


    }

    pub fn add_todo(&mut self) {
        println!("{}", "Type a todo and press [enter]:".gradient(Color::White));
        println!("{}", "------------------------------".gradient(Color::White));

        let mut input = String::from("");
        io::stdin().read_line(&mut input).expect("Failed to read line.");

        let id = (self.todos.len() + 1) as u32;

        let new_todo = TodoItem {
            id,
            description: String::from(input.trim()),
            // completed: false
        };

        self.todos.push(new_todo);
        println!("{}", "------- Todo Added -------".gradient(Color::Green));
        self.show_user_prompt();
    }

    pub fn remove_todo(&mut self) {
        println!("{}", "Please enter which todo you'd like to delete:".gradient(Color::Red));
        println!("{}", "------------------------------".gradient(Color::Red));

        let mut input = String::from("");

        io::stdin().read_line(&mut input).expect("Failed to read line.");

        let todo_index_to_delete: usize = input.trim().parse().expect("Invalid input.");

        // Minus 1 as we don't acount for 0 index when assigning ID
        self.todos.remove(todo_index_to_delete - 1);

        println!("{}", "------- Todo Deleted -------".gradient(Color::Red));

        self.show_user_prompt();
    }

    pub fn print_todos(&mut self) {
        println!("{}","Todo List".gradient(Color::Cyan));
        println!("{}", "----------------".gradient(Color::Cyan));
        for todo in &self.todos {
            println!("{}. {}", todo.id, todo.description);
        }
        println!("{}", "----------------".gradient(Color::Cyan));

        self.show_user_prompt();
    }


    pub fn clear(&mut self) {
        self.todos.clear();
        println!("{}", "------- Todo List Cleared -------".color(Color::Green));
        self.show_user_prompt();
    }
}