# TaskCLI 🦀

A command-line task manager built in Rust.

## Features
- You can add, mark done, delete, and list your taks.
- While listing tasks, you can apply a filter as per your choice, whether you wanna see tasks by ID or by status

## Installation
\`\`\`bash
git clone <repo link>
cd taskcli
cargo build
\`\`\`

## Usage
\`\`\`bash
# add a task
cargo run -- add "Buy milk"

# list all tasks
cargo run -- list --by-status/--by-id

# mark a task done
cargo run -- done 1

# delete a task
cargo run -- delete 1

# clear all tasks from storage
cargo run -- clear

# help command(to list all the available commands)
cargo run -- help

## Project Structure
\`\`\`
src/
├── main.rs      → ...
├── task.rs      → ...
├── command.rs   → ...
└── storage.rs   → ...
\`\`\`

## What I Learned
- I/O file operations
- read arguments from terminal
- read user input from terminal
- how to use modules within project
- unit tests
- ...
