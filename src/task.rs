use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
pub enum Priority {
    Low = 1,
    Medium = 2,
    High = 3,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub is_done: bool,
    pub priority: Priority,
}

impl Task {
    pub fn new(id: u32, title: String, priority: Priority) -> Self {
        Self {
            id,
            title,
            is_done: false,
            priority,
        }
    }
}
