// src/commands/list.rs

use crate::data::load_tasks;
use crate::errors::TaskError;
use crate::helpers::print_tasks; // This is the ONLY print function needed from helpers
use crate::models::{OutputFormat, Task, TomlTaskList}; // Make sure Task is also imported here
use toml::to_string_pretty;

pub fn handle_list_command(actual_format: OutputFormat) -> Result<(), TaskError> {
    let tasks = load_tasks()?;

    match actual_format {
        OutputFormat::Human => {
            if tasks.is_empty() {
                println!("No tasks yet!");
            } else {
                println!("Your tasks:");
                // Calls the comprehensive print_tasks from helpers.rs
                // It will handle numbering (starting from 1 for top-level tasks) and highlighting.
                print_tasks(&tasks, 0, None, Some(0)); // Pass Some(0) to start positional numbering
            }
        }
        OutputFormat::Json => {
            let json_string = serde_json::to_string_pretty(&tasks)?;
            println!("{}", json_string);
        }
        OutputFormat::Toml => {
            let toml_data = TomlTaskList { tasks: tasks };
            let toml_string = to_string_pretty(&toml_data)?;
            println!("{}", toml_string);
        }
    }

    Ok(())
}