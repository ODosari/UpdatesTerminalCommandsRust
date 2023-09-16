extern crate ansi_term;

use std::process::Command;
use ansi_term::Colour::{Green, White};
use ansi_term::Style;

// Function to run a shell command using Zsh
fn run_command(update_commands: &str) {
    let output = Command::new("zsh")
        .arg("-c")
        .arg(update_commands)
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        println!("Command succeeded");
    } else {
        println!("Command '{}' failed with error: {:?}", update_commands, output);
    }
}

// Main function
fn main() {
    // List of commands to run
    let commands = vec![
        "brew upgrade && brew cleanup && brew autoremove",  // Homebrew commands
        "conda update --all -y",  // Conda update
        "rustup update"  // Rust update
    ];

    // Get the total number of commands
    let total_commands = commands.len();

    // Loop through each command and run it
    for (i, command) in commands.iter().enumerate() {
        // Print the command number and the command itself in bold and green color
        let styled_message = format!(
            "{}Running update command {}{} of {}{}: {}",
            Style::new().bold().fg(White).paint("").to_string(),
            Style::new().bold().fg(Green).paint((i + 1).to_string()).to_string(),
            Style::new().bold().fg(White).paint("").to_string(),
            Style::new().bold().fg(Green).paint(total_commands.to_string()).to_string(),
            Style::new().bold().fg(White).paint("").to_string(),
            command
        );
        println!("{}", styled_message);

        // Run the command
        run_command(command);
    }
}
