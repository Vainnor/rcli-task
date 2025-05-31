use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write, Read};
use std::path::PathBuf;

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

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
    subtasks: Vec<Task>,
}

const DATA_FILE: &str = "tasks.json";

fn get_data_file_path() -> PathBuf {
    // This is a simple way to get a path. In a real application, you might
    // want to use a more robust method to find a user's data directory.
    let mut path = std::env::current_dir().unwrap();
    path.push(DATA_FILE);
    path
}

fn load_tasks() -> io::Result<Vec<Task>> {
    let path = get_data_file_path();
    if !path.exists() {
        return Ok(Vec::new());
    }

    let mut file = fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Deserialize the JSON string into a vector of Tasks
    let tasks: Vec<Task> = serde_json::from_str(&contents)?;

    Ok(tasks)
}

fn save_tasks(tasks: &Vec<Task>) -> io::Result<()> {
    let path = get_data_file_path();
    let json_string = serde_json::to_string_pretty(tasks)?; 
    
    let mut file = fs::File::create(path)?;
    file.write_all(json_string.as_bytes())?;

    Ok(())
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let mut tasks = load_tasks()?;

    match &cli.command {
        Commands::Add { description, parent_id } => {
            let new_task = Task {
                id: 0,
                description: description.clone(),
                completed: false,
                subtasks: Vec::new(),
            };

            match parent_id {
                Some(id) => {
                    if let Some(parent_task) = find_task_mut(&mut tasks, *id) {
                        let new_subtask_id = parent_task.subtasks.len() + 1;
                        let mut subtask_to_add = new_task;
                        subtask_to_add.id = new_subtask_id;
                        parent_task.subtasks.push(subtask_to_add);
                        save_tasks(&tasks)?;
                        println!("Added subtask '{}' to task {}", description, id);
                    } else {
                        println!("Error: Parent task with ID {} not found.", id);
                    }
                }
                None => {
                    let new_id = tasks.len() + 1;
                    let mut task_to_add = new_task;
                    task_to_add.id = new_id;
                    tasks.push(task_to_add);
                    save_tasks(&tasks)?;
                    println!("Added task: {}", description);
                }
            }
        }
        Commands::List => {
            if tasks.is_empty() {
                println!("No tasks yet!");
            } else {
                println!("Your tasks:");
                print_tasks(&tasks, 0);
            }
        }
        Commands::Complete { id } => {
            if let Some(task) = find_task_mut(&mut tasks, *id) {
                task.completed = true;
                mark_subtasks_complete(task);
                save_tasks(&tasks)?;
                println!("Marked task {} as complete.", id);
            } else {
                println!("Error: Task with ID {} not found.", id);
            }
        }
        Commands::Remove { id } => {
            let initial_len = count_tasks(&tasks);
            remove_task_by_id(&mut tasks, *id);
            let final_len = count_tasks(&tasks);

            if final_len < initial_len {
                reindex_tasks(&mut tasks);
                save_tasks(&tasks)?;
                println!("Removed task {}.", id);
            } else {
                println!("Error: Task with ID {} not found.", id);
            }
        }
        Commands::Edit { id, new_description } => {
            if let Some(task) = find_task_mut(&mut tasks, *id) {
                task.description = new_description.clone();
                save_tasks(&tasks)?;
                println!("Edited task {} with new description: {}", id, new_description);
            } else {
                println!("Error: Task with ID {} not found.", id);
            }
        }
    }

    Ok(())
}

fn find_task_mut(tasks: &mut Vec<Task>, id: usize) -> Option<&mut Task> {
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

fn print_tasks(tasks: &Vec<Task>, indent_level: usize) {
    let indent = "  ".repeat(indent_level); // 2 spaces per indent level
    for task in tasks {
        let status = if task.completed { "[x]" } else { "[ ]" };
        println!("{}{} {}: {}", indent, task.id, status, task.description);
        // Recursively print subtasks
        if !task.subtasks.is_empty() {
            print_tasks(&task.subtasks, indent_level + 1);
        }
    }
}

fn count_tasks(tasks: &Vec<Task>) -> usize {
    let mut count = tasks.len();
    for task in tasks {
        count += count_tasks(&task.subtasks);
    }
    count
}

fn remove_task_by_id(tasks: &mut Vec<Task>, id: usize) -> bool {
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

fn reindex_tasks(tasks: &mut Vec<Task>) {
    for (i, task) in tasks.iter_mut().enumerate() {
        task.id = i + 1;
        reindex_tasks(&mut task.subtasks); // Recursively re-index subtasks
    }
}

fn mark_subtasks_complete(task: &mut Task) {
     for subtask in task.subtasks.iter_mut() {
         subtask.completed = true;
         mark_subtasks_complete(subtask);
     }
}
