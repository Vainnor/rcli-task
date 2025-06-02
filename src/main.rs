use clap::{Parser, Subcommand, ArgAction};
use std::process;
use crate::models::OutputFormat;

mod models;
mod data;
mod commands;
mod errors;
mod helpers;
mod config;

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
        parent_id: Option<String>,
        #[arg(long, short = 'd')]
        due: Option<String>,
    },
    List {
        #[arg(long, short = 'f', value_enum)]
        format: Option<OutputFormat>,
    },
    Complete { id: String },
    Remove { id: String },
    Edit {
        id: String,
        new_description: String,
        #[arg(long, short = 'd')]
        due: Option<String>
    },
    Show {
        id: String,
        #[arg(long, short = 'f', value_enum)]
        format: Option<OutputFormat>,
    },
    SetFormat {
        #[arg(value_enum)]
        format: OutputFormat,
    },
    Archive,
    ListArchive {
        #[arg(long, short = 'f', value_enum)]
        format: Option<OutputFormat>,
    },
    Search {
        keyword: String,
        #[arg(long, short = 'a')]
        in_archive: bool,
    },
    Clear {
        #[arg(long, short = 'f', action = ArgAction::SetTrue)]
        force: bool,
    }
}

fn main() {
    let cli = Cli::parse();

    let result = match &cli.command {
        Commands::Add { description, parent_id, due } => {
            commands::add::handle_add_command(description.clone(), parent_id.clone(), due.clone())
        }
        Commands::List { format } => {
            let loaded_config = config::load_config().unwrap_or_default();
            let final_format = format.unwrap_or(loaded_config.default_output_format);
            commands::list::handle_list_command(final_format)
        }
        Commands::Show { id, format } => {
            let loaded_config = config::load_config().unwrap_or_default();
            let final_format = format.unwrap_or(loaded_config.default_output_format);
            commands::show::handle_show_command(id.clone(), final_format)
        }
        Commands::Complete { id } => {
            commands::complete::handle_complete_command(id.clone())
        }
        Commands::Remove { id } => {
            commands::remove::handle_remove_command(id.clone())
        }
        Commands::Edit { id, new_description, due } => {
            commands::edit::handle_edit_command(id.clone(), new_description.clone(), due.clone())
        }
        Commands::Clear { force } => {
            commands::clear::handle_clear_command(*force)
        }
        Commands::Archive => {
            commands::archive::handle_archive_command()
        }
        Commands::ListArchive { format } => {
            let loaded_config = config::load_config().unwrap_or_default();
            let final_format = format.unwrap_or(loaded_config.default_output_format);
            commands::list_archive::handle_list_archive_command(final_format)
        }
        Commands::Search { keyword, in_archive } => {
            commands::search::handle_search_command(keyword.clone(), *in_archive)
        }
        Commands::SetFormat { format } => {
            commands::set_format::handle_set_format_command(*format)
        }
    };

    if let Err(err) = result {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}