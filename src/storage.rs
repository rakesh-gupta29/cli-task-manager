use crate::task::Task;
use std::{fmt, fs, io};

const FILE_PATH: &str = "tasks.json";

#[derive(Debug)]
pub enum StorageError {
    Read(std::io::Error),
    Write(std::io::Error),
    Parse(serde_json::Error),
    Serialise(serde_json::Error),
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StorageError::Read(e) => write!(f, "cannot read {FILE_PATH}: {e}"),
            StorageError::Write(e) => write!(f, "cannot write {FILE_PATH}: {e}"),
            StorageError::Parse(e) => write!(f, "{FILE_PATH} is corrupted: {e}"),
            StorageError::Serialise(e) => write!(f, "cannot serialise tasks: {e}"),
        }
    }
}

// pub fn load() -> Result<Vec<Task>, StorageError> {
//     let contents = fs::read_to_string(FILE_PATH).map_err(|error| StorageError::Read(error))?;
//     let tasks = serde_json::from_str(&contents).map_err(StorageError::Parse)?;
//     Ok(tasks)
// }

pub fn load() -> Result<Vec<Task>, StorageError> {
let contents = match  fs::read_to_string(FILE_PATH) {
    Ok(c) => c,
    Err(e) =>  if  e.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
}
}

pub fn sync(tasks: &Vec<Task>) -> Result<(), StorageError> {
    let contents = serde_json::to_string_pretty(tasks).map_err(StorageError::Parse)?;
    fs::write(FILE_PATH, contents).map_err(StorageError::Write)?;
    Ok(())
}
