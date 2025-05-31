use clap::{Parser, Subcommand};
use std::io;
use std::process;
mod models;
mod data;
mod commands;
mod errors;

pub fn find_task_mut(tasks: &mut Vec<models::Task>, id: usize) -> Option<&mut models::Task> {

    for task in tasks.iter_mut() {
        if task.id == id {
            return Some(task);
        }
        if let Some(found_subtask) = find_task_mut(&mut task.subtasks, id) {
            return Some(found_subtask);
        }
    }
    None
}

pub fn find_task(tasks: &Vec<models::Task>, id: usize) -> Option<&models::Task> {

    for task in tasks.iter() {
        if task.id == id {
            return Some(task);
        }
        if let Some(found_subtask) = find_task(&task.subtasks, id) {
            return Some(found_subtask);
        }
    }
    None
}

pub fn count_tasks(tasks: &Vec<models::Task>) -> usize {

    let mut count = tasks.len();
    for task in tasks {
        count += count_tasks(&task.subtasks);
    }
    count
}

pub fn remove_task_by_id(tasks: &mut Vec<models::Task>, id: usize) -> bool {

    let initial_len = tasks.len();
    tasks.retain(|task| task.id != id);
    if tasks.len() < initial_len {
        return true;
    }

    for task in tasks.iter_mut() {
        if remove_task_by_id(&mut task.subtasks, id) {
            return true;
        }
    }

    false
}

pub fn reindex_tasks(tasks: &mut Vec<models::Task>) {

    for (i, task) in tasks.iter_mut().enumerate() {
        task.id = i + 1;
        reindex_tasks(&mut task.subtasks);
    }
}


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        description: String,
        #[arg(long, short = 's')]
        parent_id: Option<usize>,
    },
    List,
    Complete { id: usize },
    Remove { id: usize },
    Edit {
        id: usize,
        new_description: String,
    },
}


fn main() { // Change return type to ()
    let cli = Cli::parse();

    let result = match &cli.command {
        Commands::Add { description, parent_id } => {
            commands::add::handle_add_command(description.clone(), *parent_id)
        }
        Commands::List => {
            commands::list::handle_list_command()
        }
        Commands::Complete { id } => {
            commands::complete::handle_complete_command(*id)
        }
        Commands::Remove { id } => {
            commands::remove::handle_remove_command(*id)
        }
        Commands::Edit { id, new_description } => {
            commands::edit::handle_edit_command(*id, new_description.clone())
        }
    };

    // Handle the result of the command execution
    if let Err(err) = result {
        eprintln!("Error: {}", err); // Print the user-friendly error message
        process::exit(1); // Exit with a non-zero status code to indicate an error
    }
}