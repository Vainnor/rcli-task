// src/models.rs

use serde::{Deserialize, Serialize};
use clap::ValueEnum; // Add ValueEnum trait for OutputFormat
use chrono::NaiveDate;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: Uuid,
    pub description: String,
    pub completed: bool,
    pub subtasks: Vec<Task>,
    pub due_date: Option<NaiveDate>,
    #[serde(skip)] // Do NOT serialize this field
    pub display_position: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TomlTaskList {
    pub tasks: Vec<Task>,
}

// NEW: OutputFormat enum definition
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Serialize, Deserialize)] // Include Serialize/Deserialize
pub enum OutputFormat {
    #[value(name = "human")]
    Human,
    #[value(name = "json")]
    Json,
    #[value(name = "toml")] // Add this line
    Toml,
}

