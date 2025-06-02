use crate::models::{Task};
use crate::errors::TaskError;
use std::fs::{self, OpenOptions};
use std::io::{self, Read, Write};
use std::path::PathBuf;

const APP_DATA_DIR_NAME: &str = "rcli-task";
const TASKS_FILE_NAME: &str = "tasks.json"; 
const ARCHIVE_FILE_NAME: &str = "archive.json"; 
const CONFIG_FILE_NAME: &str = "config.json"; 

fn get_app_data_dir() -> Result<PathBuf, TaskError> {
    let home_dir = dirs::home_dir() 
        .ok_or_else(|| TaskError::IoError(io::Error::new(io::ErrorKind::NotFound, "Could not find home directory")))?;
    
    let app_dir = home_dir.join(APP_DATA_DIR_NAME);
    
    fs::create_dir_all(&app_dir)?;

    Ok(app_dir)
}

fn get_data_file_path() -> Result<PathBuf, TaskError> {
    Ok(get_app_data_dir()?.join(TASKS_FILE_NAME))
}

fn get_archive_file_path() -> Result<PathBuf, TaskError> {
    Ok(get_app_data_dir()?.join(ARCHIVE_FILE_NAME))
}

pub fn get_config_file_path() -> Result<PathBuf, TaskError> {
    Ok(get_app_data_dir()?.join(CONFIG_FILE_NAME))
}

pub fn load_tasks() -> Result<Vec<Task>, TaskError> {
    let path = get_data_file_path()?;

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
    let path = get_data_file_path()?;
    let json_string = serde_json::to_string_pretty(tasks)?;

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;

    file.write_all(json_string.as_bytes())?;

    Ok(())
}

pub fn load_archived_tasks() -> Result<Vec<Task>, TaskError> {
    let path = get_archive_file_path()?;

    if !path.exists() || fs::metadata(&path).map(|m| m.len()).unwrap_or(0) == 0 {
        return Ok(Vec::new());
    }

    let mut file = fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let tasks: Vec<Task> = serde_json::from_str(&contents)?;

    Ok(tasks)
}

pub fn append_to_archive(tasks_to_archive: &Vec<Task>) -> Result<(), TaskError> {
    let path = get_archive_file_path()?;
    let mut existing_tasks = load_archived_tasks()?;
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