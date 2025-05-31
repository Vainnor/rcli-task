use std::fs;
use std::io::{self, Write, Read};
use std::path::PathBuf;
use clap::{ Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { description: String },
    List,
    Complete { id: usize },
    Remove { id: usize },
}

const DATA_FILE: &str = "tasks.json";

fn get_data_file_path() -> PathBuf {
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

    let tasks: Vec<Task> = serde_json::from_str(&contents)?;

    Ok(tasks)
}

fn save_tasks(tasks: &Vec<Task>) -> io::Result<()> {
    let path = get_data_file_path();
    let json_string = serde_json::to_string(tasks)?;

    let mut file = fs::File::create(path)?;
    file.write_all(json_string.as_bytes())?;

    Ok(())
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let mut tasks = load_tasks()?;

    match &cli.command {
        Commands::Add { description } => {
            let new_id = tasks.len() + 1;
            let new_task = Task {
                id: new_id,
                description: description.clone(),
                completed: false,
            };
            tasks.push(new_task);
            save_tasks(&tasks)?;
            println!("Task added: {}", description);
        }
        Commands::List => {
            if tasks.is_empty() {
                println!("No Tasks found.");
            } else {
                println!("Tasks:");
                for task in tasks {
                    let status = if task.completed { "[x]" } else { "[ ]" };
                    println!("{}: {} {}", task.id, status, task.description);
                }
            }
        }
        Commands::Complete { id } => {
            if let Some(task) = tasks.iter_mut().find(|t| t.id == *id) {
                task.completed = true;
                save_tasks(&tasks)?;
                println!("Marked task {} as complete.", id);
            } else {
                println!("Error: Task with ID {} not found.", id);
            }
        }
        Commands::Remove { id } => {
            let initial_len = tasks.len();
            tasks.retain(|task| task.id != *id);
            if tasks.len() < initial_len {
                // Re-index tasks after removal (optional but good practice)
                for (i, task) in tasks.iter_mut().enumerate() {
                    task.id = i + 1;
                }
                save_tasks(&tasks)?;
                println!("Removed task {}.", id);
            } else {
                println!("Error: Task with ID {} not found.", id);
            }
        }
    }

    Ok(())
}