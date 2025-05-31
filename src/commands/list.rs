// src/commands/list.rs

use crate::data::load_tasks;
use crate::models::Task; // This is needed here for the print_tasks function
use crate::errors::TaskError; // Import our custom error type
use std::io;

pub fn handle_list_command() -> Result<(), TaskError> { // Updated return type
    let tasks = load_tasks()?; // Use ? operator to propagate errors

    if tasks.is_empty() {
        println!("No tasks yet!");
    } else {
        println!("Your tasks:");
        print_tasks(&tasks, 0);
    }

    Ok(())
}

// The print_tasks helper function is used here, so keep the models::Task import
fn print_tasks(tasks: &Vec<Task>, indent_level: usize) {
    let indent = "  ".repeat(indent_level);
    for task in tasks {
        let status = if task.completed { "[x]" } else { "[ ]" };
        println!("{}{} {}: {}", indent, task.id, status, task.description);
        if !task.subtasks.is_empty() {
            print_tasks(&task.subtasks, indent_level + 1);
        }
    }
}
