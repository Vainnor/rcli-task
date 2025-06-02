use crate::data::{load_tasks, save_tasks};
use crate::errors::TaskError;
use crate::helpers::resolve_task_mut;

pub fn handle_complete_command(id_prefix: String) -> Result<(), TaskError> { 
    let mut tasks = load_tasks()?;

    if let Ok(task) = resolve_task_mut(&mut tasks, &id_prefix) {
        task.completed = true;
        save_tasks(&tasks)?;
        println!("Marked task with ID prefix '{}' as complete.", id_prefix);
    } else {
        return Err(TaskError::TaskNotFound(id_prefix));
    }

    Ok(())
}