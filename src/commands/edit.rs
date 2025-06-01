// src/commands/edit.rs

use crate::data::{load_tasks, save_tasks};
use crate::errors::TaskError;
use crate::helpers::resolve_task_mut; // NEW: Import resolve_task_mut
use chrono::NaiveDate;

pub fn handle_edit_command(
    id_prefix: String, // Changed to String
    new_description: String,
    new_due_date_str: Option<String>,
) -> Result<(), TaskError> {
    let mut tasks = load_tasks()?;

    if let Ok(task) = resolve_task_mut(&mut tasks, &id_prefix) { // Use resolve_task_mut
        task.description = new_description.clone();

        if let Some(s) = new_due_date_str {
            task.due_date = Some(NaiveDate::parse_from_str(&s, "%Y-%m-%d")?);
        }

        save_tasks(&tasks)?;
        println!("Edited task with ID prefix '{}' with new description: {}", id_prefix, new_description);
    } else {
        return Err(TaskError::TaskNotFound(id_prefix));
    }

    Ok(())
}