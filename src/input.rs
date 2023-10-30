use std::io::{self, Write};

use crate::json::Todo;

pub fn create_todo() -> Todo {
    print!("Enter todo title: ");
    io::stdout().flush().unwrap();

    let mut title = String::new();
    io::stdin().read_line(&mut title).unwrap();

    let todo = Todo::new(title.trim().to_string());
    todo
}


pub fn delete_todo<'a>(todos: &'a mut Vec<Todo>) {
    print!("Enter todo ID to delete: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let id = input.trim().parse::<u64>().unwrap();
    let index = todos.iter().position(|t| t.get_id() == id);

    match index {
        Some(i) => {
            todos.remove(i);
            println!("Todo with ID {} deleted", id);
        }
        None => println!("Todo with ID {} not found", id),
    }
}

pub fn stop() {
    std::process::exit(0);
}

pub fn set_completed(todos: &mut Vec<Todo>) {
    print!("Enter todo ID to set completed: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let id = input.trim().parse::<u64>().unwrap();
    let index = todos.iter().position(|t| t.get_id() == id);

    match index {
        Some(i) => {
            todos[i].set_completed(true);
            println!("Todo with ID {} set to completed", id);
        }
        None => println!("Todo with ID {} not found", id),
    }
}
#[test]
fn test_create_todo() {
    let todo = create_todo();
    assert_eq!(todo.get_title(), "test");
    assert_eq!(todo.get_completed(), false);
}

#[test]
fn test_delete_todo() {
    let mut todos = vec![Todo::new("test".to_string())];
    println!("{:?}", &todos[0].get_id());
    delete_todo(&mut todos);
    assert_eq!(todos.len(), 0);
}