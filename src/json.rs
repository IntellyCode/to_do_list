use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::fmt;

#[derive(Deserialize,Serialize,Clone)]
pub struct Todo {
    id: u64,
    title: String,
    completed: bool,
}
impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Todo {}: {} (completed: {})", self.id, self.title, self.completed)
    }
}

impl Todo {
    pub fn get_title(&self) -> &str {
        &self.title
    }
    pub fn get_id(&self) -> u64 {
        self.id
    }
    pub fn get_completed(&self) -> bool {
        self.completed
    }
    pub fn set_completed(&mut self, completed: bool) {
        self.completed = completed;
    }
    pub fn new(title: String) -> Todo {
        let id = rand::random::<u64>();
        let todo = Todo {
            id,
            title,
            completed: false,
        };
        todo
    }
}

pub fn read_json(file_path: &str) -> Result<Vec<Todo>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let todos:Vec<Todo> = serde_json::from_reader(reader)?;
    return Ok(todos);
}

pub fn write_json(file_path: &str, todos: &[Todo]) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(file_path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, todos)?; // write updated todos to file
    Ok(())
}

#[test]
fn test_read_json() {
    let todos = read_json("todos.json").unwrap();
    assert_eq!(todos.len(), 2);
    assert_eq!(todos[0].id, 1);
    assert_eq!(todos[0].title, "Buy milk");
    assert_eq!(todos[0].completed, false);
    assert_eq!(todos[1].id, 2);
    assert_eq!(todos[1].title, "Do laundry");
    assert_eq!(todos[1].completed, true);
}

#[test]
fn test_write_json(){
    let todos=vec![
        Todo {
            id: 1,
            title: String::from("Buy milk"),
            completed: false,
        },
        Todo {
            id: 2,
            title: String::from("Do laundry"),
            completed: true,
        },
    ];
    write_json("todos.json", &todos).unwrap();
}