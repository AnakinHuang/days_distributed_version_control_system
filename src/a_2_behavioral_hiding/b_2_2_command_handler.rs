// days_dvcs/src/a_2_behavioral_hiding/b_2_2_command_handler.rs
//
//! B.2.2 Command Handler
//! The Command Handler is responsible for **executing valid commands** passed from `B.2.1 Command Parser`. It
//! interprets these commands, delegates execution to the appropriate submodules in **A.3 Repository Hiding**,
//! and formats responses using `B.2.3 Output Formatter`. This design adheres to the principle of **information hiding**,
//! where the actual repository operations are abstracted away from the Behavioral Hiding module.
//!
//! Parent Module: A.2 Behavioral Hiding
//!
//! ## Key Responsibilities
//! - Handle user commands such as initializing repositories, adding files, or switching branches.
//! - Ensure each command's execution is routed to the appropriate functionality within the Repository Hiding module.
//! - Display success or error messages to the user through the `OutputFormatter` struct from `B.2.3 Output Formatter`.
//!
//! ## Dependencies
//! This module does not depend on external libraries
//!
//! Author: Yifan (Alvin) Jiang
//! Date: 11/16/2024

use super::b_2_1_command_parser::ValidCommand;
use super::b_2_3_output_formatter::{OutputFormatter, OutputType};
use crate::a_3_repository_hiding::b_3_1_repository_management::*;
use crate::a_3_repository_hiding::b_3_2_revision_management::*;
// use crate::a_3_repository_hiding::b_3_3_branch_management::*;

use std::io;
pub struct CommandHandler;

impl CommandHandler {
    
    /// Executes the given command.
    pub fn handle_command(command: ValidCommand) -> Result<(), io::Error> {
        match command {
            ValidCommand::Init { directory } => {
                let dir = directory.unwrap_or_else(|| ".".to_string());
                println!("Initializing repository in directory: {}", dir);
                if init_repository(&dir).is_ok() {
                    OutputFormatter::display(OutputType::Success, format!("Initialized repository in directory: {}", dir));
                } else {
                    OutputFormatter::display(OutputType::Error, "Failed to initialize repository".to_string());
                }
                
                Ok(())
            }
            ValidCommand::Clone { remote_url, directory } => {
                let dir = directory.unwrap_or_else(|| ".".to_string());
                println!("Cloning repository from {} into directory: {}", remote_url, dir);
                if clone_repository(&remote_url, &dir).is_ok() {
                    OutputFormatter::display(OutputType::Success, format!("Cloned repository from {} into directory: {}", remote_url, dir));
                } else {
                    OutputFormatter::display(OutputType::Error, "Failed to clone repository".to_string());
                }
                Ok(())
            }
            ValidCommand::Add { file } => {
                // Call the file staging function
                OutputFormatter::display(OutputType::Success, format!("Added file to staging: {}", file));
                Ok(())
            }
            ValidCommand::Remove { file } => {
                // Call the file unstaging function
                OutputFormatter::display(OutputType::Success, format!("Removed file from staging: {}", file));
                Ok(())
            }
            ValidCommand::Status => {
                OutputFormatter::display(OutputType::Success, "Repository status:".to_string());
                // Call repository status function
                Ok(())
            }
            ValidCommand::Heads => {
                OutputFormatter::display(OutputType::Success, "All branches: ".to_string());
                // Call branch listing function
                Ok(())
            }
            ValidCommand::Diff { revision_1, revision_2 } => {
                OutputFormatter::display(OutputType::Success, format!("Differences between {} and {}: ", revision_1, revision_2));
                // Call diff function
                Ok(())
            }
            ValidCommand::Cat { directory, revision, file } => {
                if let Ok(content) = cat(&directory.unwrap_or(".".to_string()), &revision, &file) {
                    OutputFormatter::display(OutputType::Success, content);
                } else {
                    OutputFormatter::display(OutputType::Error, "Failed to read file".to_string());
                }
                Ok(())
            }
            ValidCommand::Checkout { branch } => {
                // Call branch checkout function
                OutputFormatter::display(OutputType::Success, format!("Switched to branch: {}", branch));
                Ok(())
            }
            ValidCommand::Commit { directory, message } => {
                println!("Committing changes with message: {}", message);
                let result = commit(&directory.unwrap_or(".".to_string()), &message);
                if result.is_err() {
                    OutputFormatter::display(OutputType::Error, format!("Failed to commit changes: {}", result.unwrap_err()));
                } else {
                    OutputFormatter::display(OutputType::Success, "Changes committed successfully".to_string());
                }
                Ok(())
            }
            ValidCommand::Log => {
                OutputFormatter::display(OutputType::Success, "Displaying commit log ...".to_string());
                // Call log function
                Ok(())
            }
            ValidCommand::Merge { branch, directory } => {
                let dir = directory.unwrap_or_else(|| ".".to_string());
                println!("Merging branch '{}' into repository in directory: {}", branch, dir);
                // Call merge function
                OutputFormatter::display(OutputType::Success, format!("Merged branch '{}' into repository in directory: {}", branch, dir));
                Ok(())
            }
            ValidCommand::Pull { directory } => {
                let dir = directory.unwrap_or_else(|| ".".to_string());
                println!("Pulling changes into directory: {}", dir);
                // Call pull function
                OutputFormatter::display(OutputType::Success, format!("Pulled changes into directory: {}", dir));
                Ok(())
            }
            ValidCommand::Push { branch, directory } => {
                let dir = directory.unwrap_or_else(|| ".".to_string());
                let br = branch.unwrap_or_else(|| "main".to_string());
                println!("Pushing branch '{}' to remote repository in directory: {}", br, dir);
                // Call push function
                OutputFormatter::display(OutputType::Success, format!("Pushed branch '{}' to remote repository in directory: {}", br, dir));
                Ok(())
            }
        }
    }
}