// src/commands/add.rs

use crate::data::{load_tasks, save_tasks};
use crate::models::Task; // No longer need usize for ID
use crate::errors::TaskError;
use crate::helpers::resolve_task_mut; // NEW: Import resolve_task_mut
use chrono::NaiveDate;
use uuid::Uuid; // NEW: Import Uuid

pub fn handle_add_command(
    description: String,
    parent_id_prefix: Option<String>, // Changed name to clarify it's a prefix string
    due_date_str: Option<String>,
) -> Result<(), TaskError> {
    let mut tasks = load_tasks()?;

    let due_date = if let Some(s) = due_date_str {
        Some(NaiveDate::parse_from_str(&s, "%Y-%m-%d")?)
    } else {
        None
    };

    let new_task = Task {
        id: Uuid::new_v4(), // NEW: Generate a UUID
        description: description.clone(),
        completed: false,
        subtasks: Vec::new(),
        due_date,
        display_position: 0, // Will be set during printing, not saved
    };

    match parent_id_prefix { // Use parent_id_prefix here
        Some(prefix) => {
            if let Ok(parent_task) = resolve_task_mut(&mut tasks, &prefix) { // Use resolve_task_mut
                parent_task.subtasks.push(new_task); // Push new task directly
                save_tasks(&tasks)?;
                println!("Added subtask '{}' to task with ID prefix '{}'.", description, prefix);
            } else {
                return Err(TaskError::ParentTaskNotFound(prefix));
            }
        }
        None => {
            tasks.push(new_task);
            save_tasks(&tasks)?;
            println!("Added task: {}", description);
        }
    }

    Ok(())
}