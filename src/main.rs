use inquire::{Text, InquireError, Select, list_option::ListOption};
use std::fmt::{self, Formatter};
use std::fs::{self, OpenOptions, File};
use std::io::Write;
use serde::{Serialize, Deserialize};
use serde_json::{Result, to_string};
use std::path::Path;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
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
#[derive(Serialize, Deserialize)]
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

    fn save_list(&self, file: File) -> std::io::Result<()> {
        // serialize the list into json
        // write to a *.json file
        let json = serde_json::to_string(&self.todos);
        let mut file = OpenOptions::new()
            .append(true)
            .open("lists/todo_list.json")?;
        file.write_all(json?.as_bytes()).unwrap();
        Ok(())
    }

    fn save(&self) {
        let file = create_file("lists/todo_list.json");
        let _ = self.save_list(file);
    }

    fn load_list(&self) {
        
    }
}

fn create_file (fname: &str) -> File {
    
    let path = Path::new(fname);
    
    if path.exists() == false {
        let mut file = File::create(fname).unwrap();
        file
    }else {
        // TODO: open the file that already exists to pass to the save_list function
    }
}   

fn create_folder (dirname: &str) {
    let path = Path::new(dirname);

    if path.exists() == false {
        fs::create_dir(dirname).unwrap();
    }
}
fn main() {
    // Goal: Terminal based ToDo app
    // todo "class" that keeps track of tasks name, and description, 
    // and whether it has been completed. bonus points for timing
    
    // TODO: Create serialization and write list to disk (serde and serde_json)
    
    create_folder("lists");

    let mut list = TodoList::new();

    // if cont == true loop continues
    let cont: bool = true;

    while cont {
        let options: Vec<_> = vec!["save list".to_string(), "add new todo".to_string(), "close list".to_string()]
            .into_iter()
            .chain(list
                    .todos
                    .iter()
                    .map(ToString::to_string))
            .collect();

        let ans = Select::new("What do you want to do?", options).raw_prompt();

        match ans {
            Ok(ListOption{value, ..}) if value == "save list" => list.save(),
            Ok(ListOption{value, ..}) if value == "add new todo" => list.create_todo(),
            Ok(ListOption{value, ..}) if value == "close list" => break,
            Ok(ListOption{index , ..}) => list.get_todo(index-2).check(),
            Err(_) => println!("Hmm, that didn't work..."),
        }
    }
}
