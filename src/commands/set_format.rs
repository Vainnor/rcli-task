// src/commands/set_format.rs

use crate::config::{load_config, save_config};
use crate::errors::TaskError;
use crate::OutputFormat;

pub fn handle_set_format_command(format: OutputFormat) -> Result<(), TaskError> {
    let mut config = load_config()?; // Load current config
    config.default_output_format = format; // Update the format
    save_config(&config)?; // Save the updated config

    println!("Default output format set to: {:?}", format);

    Ok(())
}