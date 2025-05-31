use std::fmt::{self, Display, Formatter};
use std::io; // For chaining I/O errors
use serde_json; // For chaining JSON errors

#[derive(Debug)] // Add Debug derive for easy printing of the error
pub enum TaskError {
    // Errors from file I/O
    IoError(io::Error),
    // Errors from JSON serialization/deserialization
    JsonError(serde_json::Error),
    // Application-specific errors
    TaskNotFound(usize), // Error when a task with a given ID is not found
    ParentTaskNotFound(usize), // Error when a parent task for a subtask is not found
    // Add other custom errors as needed
}

// Implement the Display trait so we can print user-friendly error messages
impl Display for TaskError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TaskError::IoError(err) => write!(f, "File system error: {}", err),
            TaskError::JsonError(err) => write!(f, "Data format error: {}", err),
            TaskError::TaskNotFound(id) => write!(f, "Error: Task with ID {} not found.", id),
            TaskError::ParentTaskNotFound(id) => write!(f, "Error: Parent task with ID {} not found.", id),
        }
    }
}

// Implement From<io::Error> and From<serde_json::Error> to easily convert
// standard library errors into our custom TaskError.
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
