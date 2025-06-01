// src/data.rs

use crate::models::{Task};
use crate::errors::TaskError;
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;

const DATA_FILE: &str = "tasks.json";
const ARCHIVE_FILE: &str = "archive.json";

fn get_data_file_path() -> PathBuf {
    std::env::current_dir().unwrap().join(DATA_FILE)
}

fn get_archive_file_path() -> PathBuf {
    std::env::current_dir().unwrap().join(ARCHIVE_FILE)
}

// Make load_tasks public
pub fn load_tasks() -> Result<Vec<Task>, TaskError> {
    let path = get_data_file_path();

    if !path.exists() || fs::metadata(&path).map(|m| m.len()).unwrap_or(0) == 0 {
        return Ok(Vec::new());
    }

    let mut file = fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let tasks: Vec<Task> = serde_json::from_str(&contents)?;

    Ok(tasks)
}

// Make load_archived_tasks public
pub fn load_archived_tasks() -> Result<Vec<Task>, TaskError> {
    let path = get_archive_file_path();

    if !path.exists() || fs::metadata(&path).map(|m| m.len()).unwrap_or(0) == 0 {
        return Ok(Vec::new());
    }

    let mut file = fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let tasks: Vec<Task> = serde_json::from_str(&contents)?;

    Ok(tasks)
}


// Make append_to_archive public
pub fn append_to_archive(tasks_to_archive: &Vec<Task>) -> Result<(), TaskError> {
    let path = get_archive_file_path();
    let mut existing_tasks = load_archived_tasks()?; // This call is within the same module, so no pub needed for load_archived_tasks here
    existing_tasks.extend(tasks_to_archive.iter().cloned());

    let json_string = serde_json::to_string_pretty(&existing_tasks)?;

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;

    file.write_all(json_string.as_bytes())?;

    Ok(())
}

// Make save_tasks public
pub fn save_tasks(tasks: &Vec<Task>) -> Result<(), TaskError> {
    let path = get_data_file_path();
    let json_string = serde_json::to_string_pretty(tasks)?;

    let mut file = fs::File::create(path)?;
    file.write_all(json_string.as_bytes())?;

    Ok(())
}