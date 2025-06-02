# rcli-task: A Powerful Command-Line Task Manager in Rust

>[!WARNING]
> This project was created in its entirety by AI. I did so to explore the current state of AI in the programing space.

`rcli-task` is a robust and user-friendly command-line interface (CLI) task manager built with the Rust programming language. It allows you to organize your tasks, add subtasks, set due dates, search, archive completed items, and customize your output – all directly from your terminal.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
    - [Adding Tasks](#adding-tasks)
    - [Listing Tasks](#listing-tasks)
    - [Showing a Specific Task](#showing-a-specific-task)
    - [Marking Tasks as Complete](#marking-tasks-as-complete)
    - [Editing Tasks](#editing-tasks)
    - [Removing Tasks](#removing-tasks)
    - [Searching Tasks](#searching-tasks)
    - [Archiving Completed Tasks](#archiving-completed-tasks)
    - [Listing Archived Tasks](#listing-archived-tasks)
    - [Clearing All Tasks](#clearing-all-tasks)
    - [Setting Default Output Format](#setting-default-output-format)
- [Data Storage](#data-storage)
- [Project Structure](#project-structure)
- [License](#license)

## Features

`rcli-task` comes packed with features designed for efficient terminal-based task management:

*   **Task Management:** Add, list, complete, edit, and remove tasks.
*   **Subtasks:** Organize complex projects with nested subtasks.
*   **Unique IDs (UUIDs):** Each task has a unique, stable ID, making manipulation reliable.
*   **User-Friendly IDs:** Interact with tasks using short, unique prefixes of their UUIDs. Positional numbering is used for easy top-level reference in lists.
*   **Due Dates:** Assign optional due dates to tasks. Overdue and due-today tasks are highlighted in your list.
*   **Color-Coded Output:** Visually distinguish between completed, pending, overdue, and due-today tasks, and easily spot search keywords.
*   **Archiving:** Move completed tasks to a separate archive, keeping your main list clean.
*   **Searching:** Find tasks by keyword across both active and archived lists.
*   **Customizable Output:** View tasks in human-readable, JSON, or TOML formats.
*   **Persistent Configuration:** Save your preferred output format.
*   **Safety Features:** Requires a `--force` flag for destructive operations like clearing all tasks.

## Installation

To install `rcli-task`, you need to have [Rust and Cargo](https://www.rust-lang.org/tools/install) installed on your system.

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/vainnor/rcli-task.git # Replace with your repo URL
    cd rcli-task
    ```
2.  **Build the project:**
    ```bash
    cargo build --release
    ```
    This will create an optimized executable in the `target/release/` directory.

3.  **Run directly (for testing):**
    ```bash
    cargo run -- <command>
    ```
    *(e.g., `cargo run -- list`)*

4.  **Install as a system command (recommended for daily use):**
    ```bash
    cargo install --path .
    ```
    This command will install `rcli-task` to your Cargo binary directory (usually `~/.cargo/bin/`), making it available directly from your terminal as `rcli-task`.

## Usage

All commands start with `rcli-task`. Use `rcli-task --help` for a full list of commands and options.

### Adding Tasks

Add a new task with a description. Optionally add it as a subtask to an existing task and set a due date.

*   **Basic Task:**
    ```bash
    rcli-task add "Buy groceries"
    ```
*   **Subtask:**
    ```bash
    rcli-task add "Milk" --parent-id <PARENT_TASK_ID_PREFIX>
    # Example: rcli-task add "Milk" --parent-id f9a1c2d4
    ```
    *(Use `rcli-task list` to find parent task ID prefixes)*
*   **With Due Date:**
    ```bash
    rcli-task add "Pay bills" --due 2025-06-30
    ```
*   **Subtask with Due Date:**
    ```bash
    rcli-task add "Draft intro" --parent-id <PARENT_ID> --due 2025-06-05
    ```

### Listing Tasks

Display all active tasks in the main list.

*   **Human-readable (default):**
    ```bash
    rcli-task list
    ```
    *(Tasks are numbered by their position for easy reference, with UUID prefix at the end)*
*   **JSON format:**
    ```bash
    rcli-task list --format json
    ```
*   **TOML format:**
    ```bash
    rcli-task list --format toml
    ```

### Showing a Specific Task

View a single task and all its subtasks.

*   **Human-readable (default):**
    ```bash
    rcli-task show <TASK_ID_PREFIX>
    # Example: rcli-task show 7b3e1f0a
    ```
*   **JSON format:**
    ```bash
    rcli-task show <TASK_ID_PREFIX> --format json
    ```
*   **TOML format:**
    ```bash
    rcli-task show <TASK_ID_PREFIX> --format toml
    ```

### Marking Tasks as Complete

Mark a task (and its subtasks, if you enabled that option in `src/helpers.rs`) as complete.

```bash
rcli-task complete <TASK_ID_PREFIX>
# Example: rcli-task complete 7b3e1f0a
```

### Editing Tasks

Change a task's description or update its due date.

*   **Change description:**
    ```bash
    rcli-task edit <TASK_ID_PREFIX> "New updated description"
    ```
*   **Update due date:**
    ```bash
    rcli-task edit <TASK_ID_PREFIX> "Original description" --due 2025-07-15
    ```
    *(Note: The description is a required argument when editing.)*

### Removing Tasks

Delete a task and all its subtasks.

```bash
rcli-task remove <TASK_ID_PREFIX>
# Example: rcli-task remove a4c5b6d7
```

### Searching Tasks

Find tasks whose descriptions contain a specific keyword.

*   **Search active tasks:**
    ```bash
    rcli-task search "report"
    ```
*   **Search active and archived tasks:**
    ```bash
    rcli-task search "project" --in-archive
    # or rcli-task search "project" -a
    ```

### Archiving Completed Tasks

Move all completed tasks from the main list to the archive.

```bash
rcli-task archive
```

### Listing Archived Tasks

View tasks that have been moved to the archive. Supports different output formats.

*   **Human-readable (default):**
    ```bash
    rcli-task list-archive
    ```
*   **JSON format:**
    ```bash
    rcli-task list-archive --format json
    ```

### Clearing All Tasks

**Permanently delete ALL tasks from the main list.** This action requires confirmation.

```bash
rcli-task clear --force
# or rcli-task clear -f
```

### Setting Default Output Format

Set your preferred output format for `list` and `show` commands.

*   **Set default to JSON:**
    ```bash
    rcli-task set-format json
    ```
*   **Set default to TOML:**
    ```bash
    rcli-task set-format toml
    ```
*   **Set default to human-readable:**
    ```bash
    rcli-task set-format human
    ```

## Data Storage

`rcli-task` stores its data in JSON files located in the directory where you run the commands:

*   `tasks.json`: Contains your active tasks.
*   `archive.json`: Contains your completed and archived tasks.
*   `config.json`: Stores your application preferences (e.g., default output format).

## Project Structure

The project follows a modular structure for maintainability:
```
rcli-task/
├── Cargo.toml          # Project dependencies and metadata
└── src/
├── main.rs         # CLI argument parsing and command dispatching
├── models.rs       # Defines core data structures (Task, OutputFormat, TomlTaskList)
├── data.rs         # Handles all file I/O operations (load/save tasks & config)
├── errors.rs       # Custom error types and their display implementations
├── helpers.rs      # General utility functions (print_tasks, ID resolution, etc.)
└── commands/       # Directory for subcommand logic
├── mod.rs      # Declares all sub-modules for commands
├── add.rs      # Logic for the 'add' command
├── list.rs     # Logic for the 'list' command
├── complete.rs # Logic for the 'complete' command
├── remove.rs   # Logic for the 'remove' command
├── edit.rs     # Logic for the 'edit' command
├── show.rs     # Logic for the 'show' command
├── clear.rs    # Logic for the 'clear' command
├── archive.rs  # Logic for the 'archive' command
├── list_archive.rs # Logic for the 'list-archive' command
├── search.rs   # Logic for the 'search' command
└── set_format.rs # Logic for the 'set-format' command
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.