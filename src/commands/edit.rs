// src/commands/edit.rs

use crate::data::{load_tasks, save_tasks};
use crate::models::Task;
use crate::errors::TaskError; // Import custom error type
use crate::find_task_mut; // Assuming find_task_mut is accessible

pub fn handle_edit_command(id: usize, new_description: String) -> Result<(), TaskError> { // Updated return type
    let mut tasks = load_tasks()?;

    if let Some(task) = find_task_mut(&mut tasks, id) {
        task.description = new_description.clone();
        save_tasks(&tasks)?; // Use ? operator
        println!("Edited task {} with new description: {}", id, new_description);
    } else {
        // Return custom error if task is not found
        return Err(TaskError::TaskNotFound(id));
    }

    Ok(())
}

// ... (find_task_mut helper function - needs to be accessible or moved)
