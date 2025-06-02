use crate::data::{load_tasks, save_tasks};
use crate::errors::TaskError;
use crate::helpers::{resolve_task_mut, remove_task_by_uuid};

pub fn handle_remove_command(id_prefix: String) -> Result<(), TaskError> {
    let mut tasks = load_tasks()?;

    let target_task_uuid = {
        let temp_tasks_ref = &mut tasks;
        let task_to_remove = resolve_task_mut(temp_tasks_ref, &id_prefix)?;
        task_to_remove.id
    };

    if remove_task_by_uuid(&mut tasks, target_task_uuid) {
        save_tasks(&tasks)?;
        println!("Removed task with ID prefix '{}'.", id_prefix);
    } else {
        return Err(TaskError::TaskNotFound(id_prefix));
    }

    Ok(())
}