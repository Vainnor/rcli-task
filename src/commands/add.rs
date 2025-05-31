// src/commands/add.rs

use crate::data::{load_tasks, save_tasks};
use crate::models::Task;
use crate::errors::TaskError; // Import custom error type
use crate::find_task_mut; // Assuming find_task_mut is accessible

pub fn handle_add_command(description: String, parent_id: Option<usize>) -> Result<(), TaskError> { // Updated return type
    let mut tasks = load_tasks()?;

    let new_task = Task {
        id: 0, // Temporary ID
        description: description.clone(),
        completed: false,
        subtasks: Vec::new(),
    };

    match parent_id {
        Some(id) => {
            if let Some(parent_task) = find_task_mut(&mut tasks, id) {
                let new_subtask_id = parent_task.subtasks.len() + 1;
                let mut subtask_to_add = new_task;
                subtask_to_add.id = new_subtask_id;
                parent_task.subtasks.push(subtask_to_add);
                save_tasks(&tasks)?; // Use ? operator
                println!("Added subtask '{}' to task {}", description, id);
            } else {
                // Return custom error if parent task is not found
                return Err(TaskError::ParentTaskNotFound(id));
            }
        }
        None => {
            let new_id = tasks.len() + 1;
            let mut task_to_add = new_task;
            task_to_add.id = new_id;
            tasks.push(task_to_add);
            save_tasks(&tasks)?; // Use ? operator
            println!("Added task: {}", description);
        }
    }

    Ok(())
}

// ... (find_task_mut helper function - needs to be accessible or moved)
