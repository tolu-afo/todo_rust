use inquire::{Text, InquireError, Select, list_option::ListOption};
use std::fmt::{self, Formatter};

#[derive(Debug)]
struct Todo {
    // a Todo Entry
    title: String,
    desc: String,
    is_completed: bool,
}

impl Todo {
    fn new(title: String, desc: String) -> Todo {
        Todo{
            title: title,
            desc: desc,
            is_completed: false,
        }

    }
    fn check(&mut self) {
        self.is_completed = !self.is_completed;
    }
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let title = &self.title;
        let desc = &self.desc;
        let check = if self.is_completed {"x"} else {" "};

        write!(f, "[{}] {} - {}", check, title, desc)
    }
}


#[derive(Debug)]
struct TodoList {
    todos: Vec<Todo>,
}

impl TodoList {
    fn new() -> TodoList {
        let new_todo_list: TodoList = TodoList {todos: Vec::new()};
        new_todo_list
    }

    fn new_todo(&mut self, title:String, desc:String) {
        self.todos.push(Todo::new(title, desc));
    }

    fn get_todo(&mut self, ind:usize) -> &mut Todo {
        let todo = self.todos.get_mut(ind);
        todo.unwrap()
    }

    fn create_todo(&mut self) {
        // creates todo using input from user.
        let title = Text::new("What is the title of your todo?: ").prompt().unwrap();
        let desc = Text::new("Describe your todo:").prompt().unwrap();

        self.new_todo(title, desc);
    }
}

fn main() {
    // Goal: Terminal based ToDo app
    // todo "class" that keeps track of tasks name, and description, 
    // and whether it has been completed. bonus points for timing

    // TODO: Create serliazation and write list to disk (serde and serde_json)
    
    let mut list = TodoList::new();

    // if cont == true loop continues
    let cont: bool = true;

    while cont {
        let options: Vec<_> = vec!["add new todo".to_string(), "close list".to_string()].into_iter().chain(list.todos.iter().map(ToString::to_string)).collect();

        let ans: Result<_, InquireError> = Select::new("What do you want to do?", options).raw_prompt();

        match ans {
            Ok(ListOption{value, ..}) if value == "add new todo" => list.create_todo(),
            Ok(ListOption{value, ..}) if value == "close list" => break,
            Ok(ListOption{index , ..}) => list.get_todo(index-2).check(),
            Err(_) => println!("Hmm, that didn't work..."),
        }
    }
}
