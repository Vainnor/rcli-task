// src/commands/remove.rs

use crate::data::{load_tasks, save_tasks};
use crate::errors::TaskError;
use crate::helpers::{resolve_task_mut, remove_task_by_uuid}; // NEW: Import resolve_task_mut and remove_task_by_uuid

pub fn handle_remove_command(id_prefix: String) -> Result<(), TaskError> { // Changed to String
    let mut tasks = load_tasks()?;

    // First, resolve the prefix to a unique UUID to know which task to remove
    let target_task_uuid = { // Scope this let-binding
        let temp_tasks_ref = &mut tasks; // Create a temporary mutable reference
        let task_to_remove = resolve_task_mut(temp_tasks_ref, &id_prefix)?;
        task_to_remove.id // Get the actual UUID
    };

    // Now, call the recursive remove function with the actual UUID
    if remove_task_by_uuid(&mut tasks, target_task_uuid) {
        // With UUIDs, we don't reindex based on position. IDs are stable.
        // No need for reindex_tasks(&mut tasks);
        save_tasks(&tasks)?;
        println!("Removed task with ID prefix '{}'.", id_prefix);
    } else {
        // This case should ideally not be reached if resolve_task_mut succeeded
        // but included for robustness
        return Err(TaskError::TaskNotFound(id_prefix));
    }

    Ok(())
}