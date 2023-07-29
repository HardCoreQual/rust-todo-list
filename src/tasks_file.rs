use crate::tasks::Tasks;

pub struct TasksFile {
    file_name: String
}

impl TasksFile {
    pub fn new(file_name: &str) -> Self {
        TasksFile { file_name: file_name.to_string() }
    }

    pub fn rewrite(&self, tasks: &Tasks) {
        std::fs::write(self.file_name.to_string(), tasks.to_string()).unwrap();
    }

    pub fn read(&self) -> Tasks {
        let tasks_str = match std::fs::read_to_string(self.file_name.to_string()) {
            Ok(content) => content,
            Err(_) => String::new(),
        };

        Tasks::from_string(&tasks_str)
    }
}
