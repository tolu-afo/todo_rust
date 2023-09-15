use clap::ValueHint;
use inquire::{Text, InquireError, Select, list_option::ListOption};
use core::panic;
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

// TODO: Make a multi todo list possible, add identifiers for lists, make saving lists and loading lists work with multiple, and add a switch list option.
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct TodoList {
    name: String,
    todos: Vec<Todo>,
}

impl TodoList {
    fn new(name: &str) -> TodoList {
        let new_todo_list: TodoList = TodoList {name: name.to_string(), todos: Vec::new()};
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

    fn save_list(&self, mut file: File) -> std::io::Result<()> {
        // serialize the list into json
        // write to a *.json file

        let json = serde_json::to_string(&self);
        file.write_all(json?.as_bytes()).unwrap();
        Ok(())
    }

    fn save(&self) {
        let path = format!("lists/{}.json", self.name);
        let file = create_file(&path);
        let _ = self.save_list(file);
    }

    fn prune_list(&mut self){
        // every todo that is checked off is deleted from the list.
        self.todos.retain(|todo| !todo.is_completed);
        self.save()
    }
}

fn list_todo_lists() -> Vec<String> {
    // Read lists dir and list out all potential lists to load
    let paths = fs::read_dir("./lists/").unwrap();
    let mut todo_lists: Vec<String> = Vec::new();

    for path in paths {
        if let Ok(entry) = path {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_file() {
                    let file_name = entry.file_name().into_string().unwrap();
                    let split_name: Vec<String> = file_name.split('.').map(|s| s.to_string()).collect();
                    if let Some(first_part) = split_name.get(0) {
                        todo_lists.push(first_part.to_string());
                    }
                }
            }
        }
    }

    todo_lists
}


fn load_list(fname: &str) -> TodoList {
    let path = format!("/lists/{}.json", fname);
    let contents = fs::read_to_string(path).expect("Unable to read file");
    let todo_list: TodoList = serde_json::from_str(&contents).expect("JSON Parsing Error");
    todo_list
}

fn load_lists() -> TodoList {
    // GOAL: Give a selection of lists to load or allow user to create new list if they choose to, or no other lists exist.
    let lists: Vec<String> = list_todo_lists();
    let mut options: Vec<String> = vec!["Create New List".to_string()];
    options.extend(lists);
    let resp = Select::new("What would you like to do?", options.clone()).raw_prompt();
    
    match resp {
        Ok(ListOption{ value, .. }) if value == "Create New List" => {
            let name = Text::new("Name your List:").prompt().unwrap();
            TodoList::new(&name)
        },
        Ok(ListOption{ value, .. }) if options.contains(&value) => {
            load_list(&value)
        },
        Ok(_) => panic!(),
        Err(_) => panic!(),
    }
}

fn create_file (fname: &str) -> File {
    
    let path = Path::new(fname);
    
    if path.exists() == false {
        let file = File::create(fname).unwrap();
        file
    }else {
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(fname)
            .unwrap();
        file
    }
}   

fn delete_file () {
    let lists: Vec<String> = list_todo_lists();
    let resp = Select::new("are you sure you want to prune, you can't reverse this action?", lists.clone()).raw_prompt();
    let file_path = format!("/lists/{}.json", resp.unwrap());
    
    match resp {
        Ok(ListOption{ value, .. }) if lists.contains(&value) => {
            let options = vec!["Yes".to_string(), "No".to_string()];
            let resp = Select::new("are you sure you want to prune, you can't reverse this action?", options).raw_prompt();
            
            match resp {
                Ok(ListOption{ value, .. }) if value == "Yes" => fs::remove_file(file_path).unwrap(),
                Ok(ListOption{ value, .. }) if value == "No" => (),
                Ok(_) => println!("Also not an answer"),
                Err(_) => println!("That's not an answer..."),
            }
        },
        Ok(_) => panic!(),
        Err(_) => panic!(),
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
    
    create_folder("lists");
    // let mut list = load_list(list_name);

    // has user select a list or create a new list.
    let mut list = load_lists();

    // if cont == true loop continues
    let cont: bool = true;

    while cont {
        let options: Vec<_> = vec!["save list".to_string(), "add new todo".to_string(), "prune list".to_string(), "switch lists".to_string(), "delete_list".to_string(), "close app".to_string()]
            .into_iter()
            .chain(list
                    .todos
                    .iter()
                    .map(ToString::to_string))
            .collect();

        let ans = Select::new("What do you want to do?", options).raw_prompt();

        match ans {
            Ok(ListOption{value, ..}) if value == "save list" => {
                list.save();
                println!("List Saved!");
            },
            Ok(ListOption{value, ..}) if value == "add new todo" => list.create_todo(),
            Ok(ListOption{value, ..}) if value == "prune list" => {
                let options = vec!["Yes".to_string(), "No".to_string()];
                let resp = Select::new("are you sure you want to prune, you can't reverse this action?", options).raw_prompt();
                
                match resp {
                    Ok(ListOption{ value, .. }) if value == "Yes" => list.prune_list(),
                    Ok(ListOption{ value, .. }) if value == "No" => continue,
                    Ok(_) => println!("Also not an answer"),
                    Err(_) => println!("That's not an answer..."),
                }
            },
            Ok(ListOption{value, ..}) if value == "switch lists" => {
                list = load_lists();
            },
            Ok(ListOption{value, ..}) if value == "close app" => break,
            Ok(ListOption{index , ..}) => list.get_todo(index-5).check(),
            Err(_) => println!("Hmm, that didn't work..."),
        }
    }
}
