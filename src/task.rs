use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub is_done: bool,
}

impl Task {
    pub fn new(id: u32, title: String) -> Self {
        Self {
            id,
            title,
            is_done: false,
        }
    }
}
