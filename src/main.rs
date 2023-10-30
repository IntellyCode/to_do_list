use json::write_json;

pub mod json;
pub mod input;
fn main() {
    let mut todos: Vec<json::Todo> = json::read_json("todos.json").unwrap_or_default();
    loop{
        println!("What would you like to do?");
        println!("1. Create a todo");
        println!("2. Delete a todo");
        println!("3. List all todos");
        println!("4. Set todo completed");
        println!("5. Exit");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        match input {
            "1" => {
                let todo = input::create_todo();
                todos.extend_from_slice(&[todo])
            }
            "2" => {
                input::delete_todo(&mut todos);
               
            }
            "3"=>{
                if todos.len() == 0 {
                    println!("No todos found");
                    continue;
                }
                for todo in &todos{
                    println!("{}",todo);
                }
            }
            "4"=>{
                input::set_completed(&mut todos);
            }
            "5" => {
                write_json("todos.json", &todos).unwrap();
                input::stop();
            }
            _ => println!("Invalid input"),
        }
    }
    
}
