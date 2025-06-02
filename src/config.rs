use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::fs;
use std::path::PathBuf;
use crate::errors::TaskError;
use crate::OutputFormat;

const CONFIG_FILE: &str = "config.json";

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

fn get_config_file_path() -> PathBuf {
    std::env::current_dir().unwrap().join(CONFIG_FILE)
}

pub fn load_config() -> Result<Config, TaskError> {
    let path = get_config_file_path();

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
    let path = get_config_file_path();
    let json_string = serde_json::to_string_pretty(config)?;

    let mut file = fs::File::create(path)?;
    file.write_all(json_string.as_bytes())?;

    Ok(())
}