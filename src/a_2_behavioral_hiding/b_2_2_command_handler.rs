// days/src/a_2_behavioral_hiding/b_2_2_command_handler.rs
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

pub struct CommandHandler;

impl CommandHandler {
    /// Executes the given command.
    pub fn handle_command(command: ValidCommand) -> Result<(), String> {
        match command {
            ValidCommand::Init { directory } => {
                let dir = directory.unwrap_or_else(|| ".".to_string());
                println!("Initializing repository in directory: {}", dir);
                // Call the repository initialization function here
                OutputFormatter::display(OutputType::Success, format!("Initialized repository in directory: {}", dir));
                Ok(())
            }
            ValidCommand::Clone { remote_url, directory } => {
                let dir = directory.unwrap_or_else(|| ".".to_string());
                println!("Cloning repository from {} into directory: {}", remote_url, dir);
                // Call repository cloning function
                OutputFormatter::display(OutputType::Success, format!("Cloned repository from {} into directory: {}", remote_url, dir));
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
            ValidCommand::Cat { file } => {
                OutputFormatter::display(OutputType::Success, format!("Content of {}: ", file));
                // Call file content display function
                Ok(())
            }
            ValidCommand::Checkout { branch } => {
                // Call branch checkout function
                OutputFormatter::display(OutputType::Success, format!("Switched to branch: {}", branch));
                Ok(())
            }
            ValidCommand::Commit { message } => {
                println!("Committing changes with message: {}", message);
                // Call commit function
                OutputFormatter::display(OutputType::Success, "Changes committed successfully".to_string());
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