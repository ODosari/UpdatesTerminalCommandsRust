# Updates Commands in Rust

## Overview

This repository contains a Rust program for running a series of update commands in a Zsh shell. The program is designed to execute commands for updating Homebrew, Conda, and Rust.

- **Name**: updates_commands_rust
- **Version**: 0.1.0
- **By**: Obaid Aldosari
- **GitHub**: [https://github.com/ODosari/UpdatesTerminalCommandsRust](https://github.com/ODosari/UpdatesTerminalCommandsRust)

## Features

- Executes a list of shell commands in a Zsh shell.
- Uses ANSI escape codes for styling the terminal output.
- Handles errors and prints appropriate messages.

## Requirements

- Rust and Cargo
- Zsh shell

## How to Run

1. **Clone the Repository**:

    ```bash
    git clone https://github.com/ODosari/UpdatesTerminalCommandsRust.git
    ```

2. **Navigate to the Project Directory**:

    ```bash
    cd UpdatesTerminalCommandsRust
    ```

3. **Build and Run the Program**:

    ```bash
    cargo run
    ```

## Adding More Commands

If you wish to add more commands to the program, you can simply extend the `commands` vector in the `main` function. For example:

```rust
let commands = vec![
    "brew upgrade && brew cleanup && brew autoremove",
    "conda update --all -y",
    "rustup update",
    "your_new_command_here"  // Add your new command here
];
```

## Code Explanation

### Importing Required Crates and Modules

The program uses Rust's `std::process::Command` for running shell commands and the `ansi_term` crate for terminal styling.

```rust
extern crate ansi_term;
use std::process::Command;
use ansi_term::Colour::{Green, White};
use ansi_term::Style;
```

### Function to Run Commands

The `run_command` function takes a shell command as an argument and executes it using the Zsh shell.

```rust
fn run_command(update_commands: &str) {
    ...
}
```

### Main Function

The `main` function contains a list of commands to run. It then loops through each command, prints it with styling, and executes it.

```rust
fn main() {
    ...
}
```

## Contributing

Feel free to fork the project, open a PR, or submit an issue.

---
