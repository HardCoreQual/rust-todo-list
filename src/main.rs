use std::io;
use tasks_file::TasksFile;
use crate::tasks::{Task, Tasks};

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
        print_tasks(&tasks, 0);
    }
}

fn print_tasks(tasks: &Tasks, intend: u8) {
    let padding = " ".repeat(intend as usize);

    for (index, task) in tasks.iter().enumerate() {
        println!(
            "{}{}. {}\n priority {}, complexity {}",
            padding,
            index + 1,
            task.value,
            task.priority,
            task.complexity,
        );
        print_tasks(&task.subtasks, intend + 1);
    }
}

fn add_task(file_name: &str) {
    println!("Write your task:");
    let mut task_str = String::new();
    io::stdin().read_line(&mut task_str).unwrap();

    println!("Write the complexity (from 0 to 255, usually 0 is less then 30 min):");
    let mut complexity_str = String::new();
    io::stdin().read_line(&mut complexity_str).unwrap();

    println!("Insert the priority (from 0 to 3):");
    let mut priority_str = String::new();
    io::stdin().read_line(&mut priority_str).unwrap();

    let tasks_file = TasksFile::new(file_name);
    let mut tasks = tasks_file.read();
    tasks.add(Task {
        value: task_str.trim().to_string(),
        complexity: complexity_str.trim().parse::<u8>().unwrap_or(255),
        priority: std::cmp::min(3, priority_str.trim().parse::<u8>().unwrap_or(0)),
        subtasks: Tasks::new()
    });

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
