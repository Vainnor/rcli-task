// src/errors.rs

use std::fmt::{self, Display, Formatter};
use std::io;
use serde_json;
use toml; // Import the toml crate to use its error type
use chrono::ParseError;

#[derive(Debug)]
pub enum TaskError {
    IoError(io::Error),
    JsonError(serde_json::Error),
    TomlError(toml::ser::Error),
    TaskNotFound(String), // Changed to String for the display ID
    ParentTaskNotFound(String), // Changed to String
    DateFormatError(String),
    ParseIntError(std::num::ParseIntError),
    AmbiguousTaskId(String), // NEW: For when a short ID matches multiple tasks
}

impl Display for TaskError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TaskError::IoError(err) => write!(f, "File system error: {}", err),
            TaskError::JsonError(err) => write!(f, "Data format error: {}", err),
            TaskError::TomlError(err) => write!(f, "TOML serialization error: {}", err),
            TaskError::TaskNotFound(id) => write!(f, "Error: Task with ID '{}' not found.", id), // Updated message
            TaskError::ParentTaskNotFound(id) => write!(f, "Error: Parent task with ID '{}' not found.", id), // Updated message
            TaskError::DateFormatError(s) => write!(f, "Invalid date format: '{}'. Expected YYYY-MM-DD.", s),
            TaskError::ParseIntError(err) => write!(f, "Invalid number format: {}", err),
            TaskError::AmbiguousTaskId(id) => write!(f, "Error: Task ID '{}' is ambiguous (matches multiple tasks). Please provide more characters.", id), // NEW
        }
    }
}

impl From<io::Error> for TaskError {
    fn from(err: io::Error) -> TaskError {
        TaskError::IoError(err)
    }
}

impl From<serde_json::Error> for TaskError {
    fn from(err: serde_json::Error) -> TaskError {
        TaskError::JsonError(err)
    }
}

// NEW: From implementation for toml::ser::Error
impl From<toml::ser::Error> for TaskError {
    fn from(err: toml::ser::Error) -> TaskError {
        TaskError::TomlError(err)
    }
}

// You might also need this if you ever deserialize TOML
// impl From<toml::de::Error> for TaskError {
//     fn from(err: toml::de::Error) -> TaskError {
//         TaskError::TomlError(err) // Re-use the same variant, or add a specific one
//     }
// }

impl From<ParseError> for TaskError {
    fn from(err: ParseError) -> TaskError {
        TaskError::DateFormatError(err.to_string()) // Convert ParseError to our custom error
    }
}

// It's also good practice to handle ParseIntError for IDs if you ever parse them from strings
impl From<std::num::ParseIntError> for TaskError {
    fn from(err: std::num::ParseIntError) -> TaskError {
        TaskError::ParseIntError(err)
    }
}