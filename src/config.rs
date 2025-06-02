use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::fs;
use std::fs::OpenOptions;
use crate::errors::TaskError;
use crate::models::OutputFormat;
use crate::data::get_config_file_path; // NEW: Import from data.rs

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub default_output_format: OutputFormat,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            default_output_format: OutputFormat::Human,
        }
    }
}

pub fn load_config() -> Result<Config, TaskError> {
    let path = get_config_file_path()?; // Use the imported function

    if !path.exists() || fs::metadata(&path).map(|m| m.len()).unwrap_or(0) == 0 {
        return Ok(Config::default());
    }

    let mut file = fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let config: Config = serde_json::from_str(&contents)?;
    Ok(config)
}

pub fn save_config(config: &Config) -> Result<(), TaskError> {
    let path = get_config_file_path()?; // Use the imported function
    let json_string = serde_json::to_string_pretty(config)?;

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;

    file.write_all(json_string.as_bytes())?;

    Ok(())
}