use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct Tasks {
    tasks: Vec<String>,
}

impl Tasks {
    pub fn new() -> Tasks {
        Tasks { tasks: Vec::new() }
    }

    pub fn from_string(content: &str) -> Tasks {
        if !content.is_empty() {
            serde_json::from_str(content).unwrap()
        } else {
            Tasks { tasks: Vec::new() }
        }
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap().to_string()
    }

    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, String> {
        self.tasks.iter()
    }

    pub fn remove(&mut self, index: i32) -> Option<String> {
        if index < 0 { return None; }
        let i = index as usize;
        if i < self.tasks.len() { Some(self.tasks.remove(i)) } else { None }
    }

    pub fn add(&mut self, value: &str) {
        self.tasks.push(value.to_string());
    }
}
