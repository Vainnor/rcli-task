// src/commands/remove.rs

use crate::data::{load_tasks, save_tasks};
use crate::models::Task;
use crate::errors::TaskError; // Import custom error type
use crate::{count_tasks, remove_task_by_id, reindex_tasks}; // Assuming these are accessible

pub fn handle_remove_command(id: usize) -> Result<(), TaskError> { // Updated return type
    let mut tasks = load_tasks()?;

    let initial_len = count_tasks(&tasks);
    let removed = remove_task_by_id(&mut tasks, id); // remove_task_by_id returns bool

    if removed {
        reindex_tasks(&mut tasks);
        save_tasks(&tasks)?; // Use ? operator
        println!("Removed task {}.", id);
    } else {
        // Return custom error if task is not found
        return Err(TaskError::TaskNotFound(id));
    }

    Ok(())
}

// ... (count_tasks, remove_task_by_id, reindex_tasks helper functions)
