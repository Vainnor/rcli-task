use std::fmt::{self, Display, Formatter};
use std::io;
use serde_json;
use toml;
use chrono::ParseError;

#[derive(Debug)]
pub enum TaskError {
    IoError(io::Error),
    JsonError(serde_json::Error),
    TomlError(toml::ser::Error),
    TaskNotFound(String),
    ParentTaskNotFound(String),
    DateFormatError(String),
    ParseIntError(std::num::ParseIntError),
    AmbiguousTaskId(String),
}

impl Display for TaskError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TaskError::IoError(err) => write!(f, "File system error: {}", err),
            TaskError::JsonError(err) => write!(f, "Data format error: {}", err),
            TaskError::TomlError(err) => write!(f, "TOML serialization error: {}", err),
            TaskError::TaskNotFound(id) => write!(f, "Error: Task with ID '{}' not found.", id),
            TaskError::ParentTaskNotFound(id) => write!(f, "Error: Parent task with ID '{}' not found.", id),
            TaskError::DateFormatError(s) => write!(f, "Invalid date format: '{}'. Expected YYYY-MM-DD.", s),
            TaskError::ParseIntError(err) => write!(f, "Invalid number format: {}", err),
            TaskError::AmbiguousTaskId(id) => write!(f, "Error: Task ID '{}' is ambiguous (matches multiple tasks). Please provide more characters.", id),
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

impl From<toml::ser::Error> for TaskError {
    fn from(err: toml::ser::Error) -> TaskError {
        TaskError::TomlError(err)
    }
}

// Deserialize TOML
// impl From<toml::de::Error> for TaskError {
//     fn from(err: toml::de::Error) -> TaskError {
//         TaskError::TomlError(err)
//     }
// }

impl From<ParseError> for TaskError {
    fn from(err: ParseError) -> TaskError {
        TaskError::DateFormatError(err.to_string())
    }
}

impl From<std::num::ParseIntError> for TaskError {
    fn from(err: std::num::ParseIntError) -> TaskError {
        TaskError::ParseIntError(err)
    }
}