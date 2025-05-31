// src/data.rs

use crate::models::Task;
use crate::errors::TaskError;
use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;

const DATA_FILE: &str = "tasks.json";

fn get_data_file_path() -> PathBuf {
    std::env::current_dir().unwrap().join(DATA_FILE) // More idiomatic way to join paths
}

pub fn load_tasks() -> Result<Vec<Task>, TaskError> {
    let path = get_data_file_path();

    // If the file doesn't exist OR is empty, return an empty vector.
    // We can check the file metadata for size.
    if !path.exists() || fs::metadata(&path).map(|m| m.len()).unwrap_or(0) == 0 {
        return Ok(Vec::new());
    }

    let mut file = fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let tasks: Vec<Task> = serde_json::from_str(&contents)?;

    Ok(tasks)
}

pub fn save_tasks(tasks: &Vec<Task>) -> Result<(), TaskError> {
    let path = get_data_file_path();
    let json_string = serde_json::to_string_pretty(tasks)?;

    let mut file = fs::File::create(path)?;
    file.write_all(json_string.as_bytes())?;

    Ok(())
}
