// src/commands/clear.rs

use crate::data::save_tasks; // Need to save an empty list
use crate::errors::TaskError;
// No other imports needed if save_tasks is the only external function used
// from other modules and Task is not directly referenced here.

pub fn handle_clear_command(force: bool) -> Result<(), TaskError> {
    if !force {
        println!("Warning: Clearing all tasks is a destructive operation.");
        println!("To proceed, use the --force (-f) flag: cargo run clear --force");
        return Ok(()); // Exit without error if not forced
    }

    // Save an empty vector of tasks, effectively clearing the file
    // The save_tasks function now expects Vec<crate::models::Task>
    // So we need to provide an empty Vec of that type.
    save_tasks(&Vec::new())?; // Correctly saves an empty vector to tasks.json

    println!("All tasks have been cleared from the main list.");

    Ok(())
}