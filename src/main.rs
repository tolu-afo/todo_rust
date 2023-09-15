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

    fn save_list(&self, mut file: File) -> std::io::Result<()> {
        // serialize the list into json
        // write to a *.json file
        let json = serde_json::to_string(&self.todos);
        file.write_all(json?.as_bytes()).unwrap();
        Ok(())
    }

    fn save(&self) {
        let file = create_file("lists/todo_list.json");
        let _ = self.save_list(file);
    }

    fn prune_list(&mut self){
        // every todo that is checked off is deleted from the list.
        self.todos.retain(|todo| !todo.is_completed);
        self.save()
    }
}

fn todos_exist() -> bool {
    // returns true if there todos already in the './lists' directory
    let paths = fs::read_dir("./lists/").unwrap();

    if let Some(path) = paths.into_iter().next() {
        true
    }else {
        false
    }
}

fn load_list(fname: &str) -> TodoList {
    if todos_exist() {
        let contents = fs::read_to_string(fname).expect("Unable to read file");
        let todo_list: TodoList = TodoList { 
            todos: serde_json::from_str(&contents).expect("JSON Parsing Error"),
        };
        todo_list
    }else {

        let todo_list = TodoList::new();
        todo_list
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

    let mut list = load_list("lists/todo_list.json");

    // if cont == true loop continues
    let cont: bool = true;

    while cont {
        let options: Vec<_> = vec!["save list".to_string(), "add new todo".to_string(), "prune list".to_string(), "close list".to_string()]
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
            Ok(ListOption{value, ..}) if value == "close list" => break,
            Ok(ListOption{index , ..}) => list.get_todo(index-4).check(),
            Err(_) => println!("Hmm, that didn't work..."),
        }
    }
}
