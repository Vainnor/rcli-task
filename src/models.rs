// src/models.rs

use serde::{Deserialize, Serialize};
use clap::ValueEnum;
use chrono::NaiveDate;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: Uuid,
    pub description: String,
    pub completed: bool,
    pub subtasks: Vec<Task>,
    pub due_date: Option<NaiveDate>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TomlTaskList {
    pub tasks: Vec<Task>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Serialize, Deserialize)] // Include Serialize/Deserialize
pub enum OutputFormat {
    #[value(name = "human")]
    Human,
    #[value(name = "json")]
    Json,
    #[value(name = "toml")]
    Toml,
}

