use crate::task::Task;
use std::fs;

const FILE_PATH: &str = "tasks.json";

pub fn load() -> Vec<Task> {
    match fs::read_to_string(FILE_PATH) {
        Ok(contents) => serde_json::from_str(&contents).unwrap_or_default(),
        Err(_) => Vec::new(),
    }
}

pub fn sync(tasks: &Vec<Task>) {
    let contents = serde_json::to_string_pretty(tasks).unwrap();
    fs::write(FILE_PATH, contents).unwrap();
}
