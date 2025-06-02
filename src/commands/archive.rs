use crate::data::{load_tasks, save_tasks, append_to_archive};
use crate::errors::TaskError;

pub fn handle_archive_command() -> Result<(), TaskError> {
    let tasks = load_tasks()?;

    let mut incomplete_tasks = Vec::new();
    let mut completed_tasks = Vec::new();

    for task in tasks {
        if task.completed {
            completed_tasks.push(task);
        } else {
            incomplete_tasks.push(task);
        }
    }
    
    save_tasks(&incomplete_tasks)?;
    println!("Incomplete tasks saved to main list.");

    if !completed_tasks.is_empty() {
        append_to_archive(&completed_tasks)?;
        println!("Archived {} completed tasks.", completed_tasks.len());
    } else {
        println!("No completed tasks to archive.");
    }

    Ok(())
}