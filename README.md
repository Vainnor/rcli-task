# rcli-task

`rcli-task` is a simple command-line task manager built with Rust. It allows you to add, list, mark as complete, and remove tasks directly from your terminal. Tasks are stored in a local JSON file.

## Features

*   Add new tasks with a description.
*   List all pending and completed tasks.
*   Mark tasks as complete.
*   Remove tasks.

## Getting Started

### Prerequisites

*   [Rust](https://www.rust-lang.org/tools/install) and [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed on your system.

### Installation

1.  Clone the repository:

    ```bash
    git clone https://github.com/Vainnor/rcli-task.git
    cd rcli-task
    ```

2.  Build the project:

    ```bash
    cargo build --release
    ```

    The `--release` flag builds an optimized executable.

3.  The executable will be located in the `target/release/` directory. You can either run it from there or copy it to a directory in your system's PATH to run it from anywhere.

    For example, to run from the release directory:

    ```bash
    ./target/release/rcli-task add "My first task"
    ```

    Or, to copy it to a PATH directory (like `/usr/local/bin` on macOS/Linux - you might need `sudo`):

    ```bash
    cp target/release/rcli-task /usr/local/bin/
    ```

    Now you can run it from anywhere:

    ```bash
    rcli-task add "My first task"
    ```

## Usage

The `rcli-task` command uses subcommands for different actions.

### Adding a Task

To add a new task, use the `add` subcommand followed by the task description. The description should be enclosed in quotes if it contains spaces.

```bash
rcli-task add "Remember to buy milk"
rcli-task add "Go for a run"
```

### Listing Tasks

To list all tasks, use the `list` subcommand.

```bash
rcli-task list
```

This will display your tasks with their ID, status (`[ ]` for pending, `[x]` for complete), and description.

