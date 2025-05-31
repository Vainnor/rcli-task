// src/commands/complete.rs

use crate::data::{load_tasks, save_tasks};
use crate::models::Task;
use crate::errors::TaskError; // Import custom error type
use crate::find_task_mut; // Assuming find_task_mut is accessible

pub fn handle_complete_command(id: usize) -> Result<(), TaskError> { // Updated return type
    let mut tasks = load_tasks()?;

    if let Some(task) = find_task_mut(&mut tasks, id) {
        task.completed = true;
        // mark_subtasks_complete(task);
        save_tasks(&tasks)?; // Use ? operator
        println!("Marked task {} as complete.", id);
    } else {
        // Return custom error if task is not found
        return Err(TaskError::TaskNotFound(id));
    }

    Ok(())
}

// ... (find_task_mut helper function - needs to be accessible or moved)
