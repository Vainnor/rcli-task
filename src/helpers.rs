use crate::models::Task;
use colored::Colorize;
use chrono::Local;
use uuid::Uuid;
use crate::errors::TaskError;

pub fn print_tasks(
    tasks: &[Task],
    indent_level: usize,
    highlight_keyword: Option<&str>,
    initial_position_counter: Option<usize>,
) {
    let indent = "    ".repeat(indent_level);
    let today = Local::now().naive_local().date();

    let mut current_display_index = initial_position_counter.unwrap_or(0);

    for task in tasks {
        let status_str = if task.completed { "[x]" } else { "[ ]" };
        let mut task_description_formatted = format!("{}", task.description);

        if let Some(keyword) = highlight_keyword {
            let mut output_parts: Vec<String> = Vec::new();
            let mut current_char_idx = 0;
            let lower_description = task_description_formatted.to_lowercase();
            let lower_keyword = keyword.to_lowercase();

            if lower_keyword.is_empty() {
                output_parts.push(task_description_formatted.clone());
            } else {
                while let Some(match_start) =
                    lower_description[current_char_idx..].find(&lower_keyword)
                {
                    let actual_match_start = current_char_idx + match_start;
                    let actual_match_end = actual_match_start + lower_keyword.len();

                    output_parts.push(
                        task_description_formatted[current_char_idx..actual_match_start]
                            .to_string(),
                    );
                    output_parts.push(
                        task_description_formatted[actual_match_start..actual_match_end]
                            .red()
                            .bold()
                            .underline()
                            .to_string(),
                    );

                    current_char_idx = actual_match_end;
                }
                output_parts.push(task_description_formatted[current_char_idx..].to_string());
            }
            task_description_formatted = output_parts.join("");
        }

        let mut date_suffix = String::new();
        if let Some(due_date) = task.due_date {
            date_suffix = format!(" (Due: {})", due_date.format("%Y-%m-%d"));
            if !task.completed && due_date < today {
                date_suffix = date_suffix.red().bold().to_string();
            } else if !task.completed && due_date == today {
                date_suffix = date_suffix.yellow().bold().to_string();
            }
        }

        let mut printed_line_content = String::new();

        if indent_level == 0 && initial_position_counter.is_some() {
            current_display_index += 1;
            printed_line_content.push_str(&format!("{}: ", current_display_index));
        }

        printed_line_content.push_str(&format!(
            "{}{} {}{}",
            indent, status_str, task_description_formatted, date_suffix
        ));

        let final_colored_line: String;
        if task.completed {
            final_colored_line = printed_line_content.green().strikethrough().to_string();
        } else {
            final_colored_line = printed_line_content.white().to_string();
        }

        println!(
            "{} {} {}",
            final_colored_line,
            "-".dimmed(),
            task.id.simple().to_string().chars().take(8).collect::<String>().dimmed(),
        );

        if !task.subtasks.is_empty() {
            print_tasks(&task.subtasks, indent_level + 1, highlight_keyword, None);
        }
    }
}

pub fn resolve_task_mut<'a>(
    tasks: &'a mut Vec<Task>,
    id_prefix: &str,
) -> Result<&'a mut Task, TaskError> {
    let mut found_tasks: Vec<&'a mut Task> = Vec::new();
    find_matching_tasks_by_id_prefix(tasks, id_prefix, &mut found_tasks);

    if found_tasks.len() == 1 {
        Ok(found_tasks.pop().unwrap())
    } else if found_tasks.is_empty() {
        Err(TaskError::TaskNotFound(id_prefix.to_string()))
    } else {
        Err(TaskError::AmbiguousTaskId(id_prefix.to_string()))
    }
}

fn find_matching_tasks_by_id_prefix<'a>(
    tasks_to_search: &'a mut Vec<Task>,
    id_prefix: &str,
    results: &mut Vec<&'a mut Task>,
) {
    for task in tasks_to_search.iter_mut() {
        let id_matches = task.id.simple().to_string().starts_with(id_prefix);
        if !task.subtasks.is_empty() {
            find_matching_tasks_by_id_prefix(&mut task.subtasks, id_prefix, results);
        }
        else if id_matches {
            results.push(task);
        }
    }
}
pub fn resolve_task<'a>(
    tasks: &'a Vec<Task>,
    id_prefix: &str,
) -> Result<&'a Task, TaskError> {
    let mut found_tasks: Vec<&Task> = Vec::new();
    find_matching_tasks_by_id_prefix_immutable(tasks, id_prefix, &mut found_tasks);

    if found_tasks.len() == 1 {
        Ok(found_tasks.pop().unwrap())
    } else if found_tasks.is_empty() {
        Err(TaskError::TaskNotFound(id_prefix.to_string()))
    } else {
        Err(TaskError::AmbiguousTaskId(id_prefix.to_string()))
    }
}

fn find_matching_tasks_by_id_prefix_immutable<'a>(
    tasks_to_search: &'a Vec<Task>,
    id_prefix: &str,
    results: &mut Vec<&'a Task>,
) {
    for task in tasks_to_search.iter() {
        if task.id.simple().to_string().starts_with(id_prefix) {
            results.push(task);
        }
        if !task.subtasks.is_empty() {
            find_matching_tasks_by_id_prefix_immutable(&task.subtasks, id_prefix, results);
        }
    }
}

pub fn remove_task_by_uuid(tasks: &mut Vec<Task>, target_uuid: Uuid) -> bool {
    let initial_len = tasks.len();
    tasks.retain(|task| task.id != target_uuid);
    if tasks.len() < initial_len {
        return true;
    }

    for task in tasks.iter_mut() {
        if remove_task_by_uuid(&mut task.subtasks, target_uuid) {
            return true;
        }
    }
    false
}