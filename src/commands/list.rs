use crate::data::load_tasks;
use crate::errors::TaskError;
use crate::helpers::print_tasks;
use crate::models::{OutputFormat, TomlTaskList};
use toml::to_string_pretty;

pub fn handle_list_command(actual_format: OutputFormat) -> Result<(), TaskError> {
    let tasks = load_tasks()?;

    match actual_format {
        OutputFormat::Human => {
            if tasks.is_empty() {
                println!("No tasks yet!");
            } else {
                println!("Your tasks:");
                print_tasks(&tasks, 0, None, Some(0)); 
            }
        }
        OutputFormat::Json => {
            let json_string = serde_json::to_string_pretty(&tasks)?;
            println!("{}", json_string);
        }
        OutputFormat::Toml => {
            let toml_data = TomlTaskList { tasks };
            let toml_string = to_string_pretty(&toml_data)?;
            println!("{}", toml_string);
        }
    }

    Ok(())
}