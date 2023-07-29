use std::io;
use tasks_file::TasksFile;

mod tasks;
mod tasks_file;

fn main() {
    let file_name = "tasks.json";

    loop {
        println!("What do you want to do?");
        println!("1. List tasks");
        println!("2. Add a task");
        println!("3. Remove task");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let choice: u8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            },
        };

        match choice {
            1 => list_tasks(&file_name),
            2 => add_task(&file_name),
            3 => remove_task(&file_name),
            _ => println!("Please enter a valid number"),
        }
    }
}

fn list_tasks(file_name: &str) {
    let tasks = TasksFile::new(file_name).read();

    if tasks.is_empty() {
        println!("You have no tasks");
    } else {
        println!("You have the following tasks:");
        for (index, task) in tasks.iter().enumerate() {
            println!("{}. {}", index + 1, task);
        }
    }
}


fn add_task(file_name: &str) {
    println!("Write your task:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let task = input.trim().to_string();

    let tasks_file = TasksFile::new(file_name);
    let mut tasks = tasks_file.read();
    tasks.add(&task);

    tasks_file.rewrite(&tasks);
}

fn remove_task(file_name: &str) {
    println!("This is existing files:");
    list_tasks(file_name);
    println!("\nWhich number you want to remove?");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if let Ok(num) = input.trim().parse::<i32>() {
        let tasks_file = TasksFile::new(file_name);
        let mut tasks = tasks_file.read();
        let index = num - 1;
        if index > -1 {
            tasks.remove(index);
        }
        tasks_file.rewrite(&tasks);
    }
}
