use crate::data::load_archived_tasks;
use crate::errors::TaskError;
use crate::helpers::print_tasks;
use crate::models::{OutputFormat, TomlTaskList};

pub fn handle_list_archive_command(actual_format: OutputFormat) -> Result<(), TaskError> {
    let tasks = load_archived_tasks()?;

    match actual_format {
        OutputFormat::Human => {
            if tasks.is_empty() {
                println!("No archived tasks yet!");
            } else {
                println!("Your archived tasks:");
                print_tasks(&tasks, 0, None, Some(0));
            }
        }
        OutputFormat::Json => {
            let json_string = serde_json::to_string_pretty(&tasks)?;
            println!("{}", json_string);
        }
        OutputFormat::Toml => {
            let toml_data = TomlTaskList { tasks };
            let toml_string = toml::to_string_pretty(&toml_data)?;
            println!("{}", toml_string);
        }
    }

    Ok(())
}