use std::io;
use tasks_file::TasksFile;
use crate::tasks::{Task, Tasks, TaskStatus};

mod tasks;
mod tasks_file;

fn main() {
    let file_name = "tasks.json";

    loop {
        println!("What do you want to do?");
        println!("1. List of tasks");
        println!("2. Add a task");
        println!("3. Remove task");
        println!("4. Edit task");
        println!("5. List of closed tasks");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let choice: u8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            },
        };

        let task_cli = TasksCLI::new(&file_name);

        match choice {
            1 => task_cli.list_tasks(),
            2 => task_cli.add_task(),
            3 => task_cli.remove_task(),
            _ => println!("Please enter a valid number"),
        }
    }
}

struct TasksCLI {
    tasks_file: TasksFile,
}

impl TasksCLI {
    fn new(file_name: &str) -> Self {
        TasksCLI { tasks_file: TasksFile::new(file_name) }
    }

    fn list_tasks(&self) {
        let tasks = self.tasks_file.read();

        if tasks.is_empty() {
            println!("You have no tasks");
        } else {
            println!("You have the following tasks:");
            self.print_tasks_deep(&tasks, 0);
        }
    }

    fn print_tasks_deep(&self, tasks: &Tasks, intend: u8) {
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
            self.print_tasks_deep(&task.subtasks, intend + 1);
        }
    }

    fn add_task(&self) {
        println!("Write your task:");
        let mut task_str = String::new();
        io::stdin().read_line(&mut task_str).unwrap();

        println!("Write the complexity (from 0 to 255, usually 0 is less then 30 min):");
        let mut complexity_str = String::new();
        io::stdin().read_line(&mut complexity_str).unwrap();

        println!("Insert the priority (from 0 to 3):");
        let mut priority_str = String::new();
        io::stdin().read_line(&mut priority_str).unwrap();

        let mut tasks = self.tasks_file.read();
        tasks.add(Task {
            value: task_str.trim().to_string(),
            complexity: complexity_str.trim().parse::<u8>().unwrap_or(255),
            priority: std::cmp::min(3, priority_str.trim().parse::<u8>().unwrap_or(0)),
            subtasks: Tasks::new(),
            status: TaskStatus::Undefine,
        });

        self.tasks_file.rewrite(&tasks);
    }

    fn remove_task(&self) {
        println!("This is existing files:");
        self.list_tasks();
        println!("\nWhich number you want to remove?");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if let Ok(num) = input.trim().parse::<i32>() {
            let mut tasks = self.tasks_file.read();
            let index = num - 1;
            if index > -1 {
                tasks.remove(index);
            }
            self.tasks_file.rewrite(&tasks);
        }
    }
}
