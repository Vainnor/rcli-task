use crate::data::save_tasks;
use crate::errors::TaskError;
pub fn handle_clear_command(force: bool) -> Result<(), TaskError> {
    if !force {
        println!("Warning: Clearing all tasks is a destructive operation.");
        println!("To proceed, use the --force (-f) flag: cargo run clear --force");
        return Ok(()); 
    }

    save_tasks(&Vec::new())?;
    
    println!("All tasks have been cleared from the main list.");

    Ok(())
}