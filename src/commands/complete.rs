// src/commands/complete.rs

use crate::data::{load_tasks, save_tasks};
use crate::errors::TaskError;
use crate::helpers::resolve_task_mut; // NEW: Import resolve_task_mut
// No direct Task import needed here for task manipulation

pub fn handle_complete_command(id_prefix: String) -> Result<(), TaskError> { // Changed to String
    let mut tasks = load_tasks()?;

    if let Ok(task) = resolve_task_mut(&mut tasks, &id_prefix) { // Use resolve_task_mut
        task.completed = true;
        // mark_subtasks_complete(task); // If you use this, adapt it
        save_tasks(&tasks)?;
        println!("Marked task with ID prefix '{}' as complete.", id_prefix);
    } else {
        return Err(TaskError::TaskNotFound(id_prefix));
    }

    Ok(())
}