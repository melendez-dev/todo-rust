use std::io;
use tasks::Tasks;
mod tasks;
fn main() {
    // presentation
    print!("#######################################\n");
    print!("Welcome to TODO CLI written in RUST\n");
    print!("#######################################\n");

    let mut tasks: Vec<Tasks> = Vec::new();

    loop {
        println!("\n 1. Add a task\n 2. List tasks\n 3. complete a task\n 4. Update a task\n 5. Delete a task\n 6. Exit\n");

        let mut choice: String = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("error to read the line");

        match choice.trim() {
            "1" => {
                tasks::add_task(&mut tasks);
            }

            "2" => {
                tasks::list_tasks(&tasks);
            }

            "3" => {
                tasks::complete_task(&mut tasks);
            }

            "4" => {
                tasks::update_tasks(&mut tasks);
            }

            "5" => {
                tasks::delete_task(&mut tasks);
            }

            "6" => {
                break;
            }
            _ => {
                println!("Not valid argument");
            }
        }
    }
}
