use crate::data::load_tasks;
use crate::models::{OutputFormat, TomlTaskList};
use crate::errors::TaskError;
use crate::helpers::{print_tasks, resolve_task}; // NEW: Import resolve_task (immutable version)
use toml::to_string_pretty;

pub fn handle_show_command(id_prefix: String, actual_format: OutputFormat) -> Result<(), TaskError> {
    let tasks = load_tasks()?;

    if let Ok(task) = resolve_task(&tasks, &id_prefix) {
        match actual_format {
            OutputFormat::Human => {
                println!("Task with ID prefix '{}':", id_prefix);
                let task_owned = task.clone();
                print_tasks(&[task_owned], 0, None, Some(0)); // Pass Some(0) if you want it numbered here too, or None if you don't want a number
            }
            OutputFormat::Json => {
                let json_string = serde_json::to_string_pretty(&task)?;
                println!("{}", json_string);
            }
            OutputFormat::Toml => {
                let toml_data = TomlTaskList { tasks: vec![task.clone()] };
                let toml_string = to_string_pretty(&toml_data)?;
                println!("{}", toml_string);
            }
        }
    } else {
        return Err(TaskError::TaskNotFound(id_prefix));
    }

    Ok(())
}