pub struct Tasks {
    tasks: Vec<String>,
}

impl Tasks {
    pub fn new() -> Tasks {
        Tasks { tasks: Vec::new() }
    }

    pub fn from_content(content: &str) -> Tasks {
        let tasks = content
            .lines()
            .enumerate()
            .filter(|&(_, line)| !line.is_empty())
            .map(|(_, line)| line.trim().to_string())
            .collect();

        Tasks { tasks }
    }

    pub fn to_content(&self) -> String {
        self.tasks.join("\n")
    }

    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, String> {
        self.tasks.iter()
    }

    pub fn remove(&mut self, index: i32) -> String {
        let a: u8 = index as u8;
        self.tasks.remove(a as usize)
    }

    pub fn add(&mut self, value: &str) {
        self.tasks.push(value.to_string());
    }
}
