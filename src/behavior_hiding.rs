// CSC 253/453 DVCS Assignment

use crate::repository_hiding::{RepoError, Repository};
use std::io;

#[allow(dead_code)]
pub enum Command {
    Init,
    Clone { repo_url: String },
    Add { file_path: String },
    Remove { file_path: String },
    Status,
    Heads,
    Diff { file_path: Option<String> },
    Cat { file_name: String },
    Checkout { branch_name: String },
    Commit { commit_message: String },
    Log,
    Merge { branch_name: String },
    Pull,
    Push,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum BehaviorError {
    IoError(io::Error),
    RepoError(RepoError),
    CommandError(String),
}

impl From<io::Error> for BehaviorError {
    fn from(err: io::Error) -> BehaviorError {
        BehaviorError::IoError(err)
    }
}

impl From<RepoError> for BehaviorError {
    fn from(err: RepoError) -> BehaviorError {
        BehaviorError::RepoError(err)
    }
}

#[allow(dead_code)]
pub enum Error {
    ParseError(String),
    ValidationError(String),
    ExecutionError(String),
}

#[allow(dead_code)]
pub enum OutputStyle {
    Plain,
    Error,
    Success,
}

#[allow(dead_code)]
pub struct ValidCommand {
    pub command: Command,
}

#[allow(dead_code)]
pub trait Valid {
    fn is_valid(&self) -> bool;
}

pub struct BehaviorHiding;

#[allow(dead_code)]
impl BehaviorHiding {
    /// Parses a command from user input.
    pub fn parse_command(input: &str) -> Result<String, BehaviorError> {
        if input.trim().is_empty() {
            return Err(BehaviorError::CommandError("Empty command".to_string()));
        }
        Ok(input.trim().to_string())
    }

    /// Validates a command against known DVCS commands.
    pub fn validate_command(command: &str) -> Result<&str, BehaviorError> {
        let valid_commands = ["init", "commit", "checkout", "status"];
        if valid_commands.contains(&command) {
            Ok(command)
        } else {
            Err(BehaviorError::CommandError(format!(
                "Invalid command: {}",
                command
            )))
        }
    }

    /// Executes a command by calling appropriate module functions.
    pub fn execute_command(command: &str, directory: &str) -> Result<(), BehaviorError> {
        match command {
            "init" => {
                Repository::init(directory).map_err(|e| BehaviorError::RepoError(e))?;
                println!("Repository initialized in {}", directory);
            }
            "commit" => {
                Repository::commit(directory, "Initial commit")
                    .map_err(|e| BehaviorError::RepoError(e))?;
                println!("Commit created in {}", directory);
            }
            "checkout" => {
                println!("Enter the commit ID or branch name to checkout:");
                let mut buffer = String::new();
                io::stdin().read_line(&mut buffer)?;
                let target = buffer.trim();
                Repository::checkout(directory, target).map_err(|e| BehaviorError::RepoError(e))?;
                println!("Checked out to {}", target);
            }
            "status" => {
                let status =
                    Repository::status(directory).map_err(|e| BehaviorError::RepoError(e))?;
                println!("Repository status:\n{}", status);
            }
            _ => {
                return Err(BehaviorError::CommandError(
                    "Command execution failed".to_string(),
                ));
            }
        }
        Ok(())
    }

    /// Displays output for the executed command.
    pub fn display_output(output: &str, style: &str) {
        match style {
            "success" => println!("\x1b[32m{}\x1b[0m", output), // Green text for success
            "error" => println!("\x1b[31m{}\x1b[0m", output),   // Red text for errors
            _ => println!("{}", output),                        // Default
        }
    }
}
