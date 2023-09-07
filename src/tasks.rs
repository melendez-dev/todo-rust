use std::io;

pub struct Tasks {
    description: String,
    complete: bool,
}

// add task
pub fn add_task(task: &mut Vec<Tasks>) {
    let mut description: String = String::new();
    println!("Write the description about the task");
    io::stdin()
        .read_line(&mut description)
        .expect("error to read line");

    let new_task: Tasks = Tasks {
        description,
        complete: false,
    };

    task.push(new_task)
}

// list tasks
pub fn list_tasks(tasks: &Vec<Tasks>) {
    for (_index, item) in tasks.iter().enumerate() {
        println!(
            "description: {}state: {}",
            item.description,
            if item.complete {
                "Completa"
            } else {
                "Pendiente"
            }
        );
    }
}

// complete tasks
pub fn complete_task(tasks: &mut Vec<Tasks>) {
    // get the index of the task to mark like complete
    let mut index: String = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("error to read the line");

    // convert to int
    let index: usize = index.trim().parse().expect("error to parse to int");

    if index > 0 && index <= tasks.len() {
        tasks[index - 1].complete = true;
        println!("completed tasks");
    } else {
        println!("Tasks not found");
    }
}

// update tasks
pub fn update_tasks(tasks: &mut Vec<Tasks>) {
    // know what index wants to modifity
    let mut index: String = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("error to read the line");

    // convert to int
    let index: usize = index.trim().parse().expect("error to parse to int");

    if index > 0 && index <= tasks.len() {
        let mut new_description: String = String::new();

        io::stdin()
            .read_line(&mut new_description)
            .expect("error to read the line");

        tasks[index - 1].description = new_description;
    } else {
        println!("Tasks not found");
    }
}

pub fn delete_task(tasks: &mut Vec<Tasks>) {
    let mut index: String = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("error to read the line");

    // convert to int
    let index: usize = index.trim().parse().expect("error to parse to int");

    tasks.remove(index - 1);
}
