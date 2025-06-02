use crate::data::{load_tasks, load_archived_tasks};
use crate::errors::TaskError;
use crate::models::Task;
use crate::helpers::print_tasks;
use colored::Colorize;

pub fn handle_search_command(keyword: String, in_archive: bool) -> Result<(), TaskError> {
    println!("Searching for '{}'...", keyword.yellow().bold());

    let active_tasks = load_tasks()?;
    let mut found_tasks = Vec::new();
    
    find_matching_tasks(&active_tasks, &keyword, &mut found_tasks);

    if in_archive {
        let archived_tasks = load_archived_tasks()?;
        find_matching_tasks(&archived_tasks, &keyword, &mut found_tasks);
    }

    if found_tasks.is_empty() {
        println!("No tasks found matching '{}'.", keyword.yellow().bold());
    } else {
        println!("Found tasks:");
        print_tasks(&found_tasks, 0, Some(&keyword), Some(0)); // Pass Some(0) for numbering search results
    }
    Ok(())
}

fn find_matching_tasks(
    tasks_to_search: &[Task],
    keyword: &str,
    results: &mut Vec<Task>,
) {
    let lower_keyword_owned = keyword.to_lowercase();
    let lower_keyword_ref = lower_keyword_owned.as_str();

    for task in tasks_to_search {
        let lower_description_owned = task.description.to_lowercase();
        let lower_description_ref = lower_description_owned.as_str();
        if lower_description_ref.contains(lower_keyword_ref) {
            results.push(task.clone());
        }
        if !task.subtasks.is_empty() {
            find_matching_tasks(&task.subtasks, keyword, results);
        }
    }
}