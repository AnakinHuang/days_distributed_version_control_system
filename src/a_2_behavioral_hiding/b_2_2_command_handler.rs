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

use crate::a_1_file_system_hiding::REMOTE;
use crate::a_3_repository_hiding::{
    b_3_1_repository_management::*, b_3_2_revision_management::*, b_3_3_branch_management::*,
    b_3_4_synchronization_handler::*, b_3_5_cross_revision_management::*,
};

pub struct CommandHandler;

impl CommandHandler {
    /// Executes the given command.

    pub fn handle_command(command: ValidCommand) {
        match command {
            ValidCommand::Init { directory } => {
                OutputFormatter::display(
                    OutputType::Process,
                    format!("Initializing repository in directory: '{}'", directory),
                );
                let result = init_repository(&directory, true);
                if result.is_ok() {
                    OutputFormatter::display(
                        OutputType::Success,
                        format!("Initialized repository in directory: '{}'", directory),
                    )
                } else {
                    OutputFormatter::display(
                        OutputType::Error,
                        format!(
                            "Failed to initialize repository in '{}': {}",
                            directory,
                            result.unwrap_err()
                        ),
                    )
                }
            }
            ValidCommand::Clone { repo, directory } => {
                OutputFormatter::display(
                    OutputType::Process,
                    format!("Cloning repository from '{}' to '{}'", repo, directory),
                );
                let result = clone_repository(&repo, &directory);
                if result.is_ok() {
                    OutputFormatter::display(
                        OutputType::Success,
                        format!("Cloned repository from '{}' to '{}'", repo, directory),
                    );
                } else {
                    OutputFormatter::display(
                        OutputType::Error,
                        format!(
                            "Failed to clone repository from '{}' to '{}': {}",
                            repo,
                            directory,
                            result.unwrap_err()
                        ),
                    );
                }
            }
            ValidCommand::Add { pathspec } => {
                let files = pathspec
                    .iter()
                    .map(|f| format!("'{}'", f))
                    .collect::<Vec<String>>()
                    .join(" ");
                OutputFormatter::display(OutputType::Process, format!("Adding file: {}", files));
                let result = add(".", pathspec);
                if result.is_ok() {
                    OutputFormatter::display(OutputType::Success, format!("Added file: {}", files));
                } else {
                    OutputFormatter::display(
                        OutputType::Error,
                        format!("Failed to add {}: {}", files, result.unwrap_err()),
                    );
                }
            }
            ValidCommand::Remove { pathspec } => {
                let files = pathspec
                    .iter()
                    .map(|f| format!("'{}'", f))
                    .collect::<Vec<String>>()
                    .join(" ");
                OutputFormatter::display(OutputType::Process, format!("Removing file: {}", files));
                let result = remove(".", pathspec);
                if result.is_ok() {
                    OutputFormatter::display(
                        OutputType::Success,
                        format!("Removed file: {}", files),
                    );
                } else {
                    OutputFormatter::display(
                        OutputType::Error,
                        format!("Failed to remove {}: {}", files, result.unwrap_err()),
                    );
                }
            }
            ValidCommand::Status { repo } => {
                OutputFormatter::display(
                    OutputType::Process,
                    format!("Checking status of repository: '{}'", repo),
                );
                let result = status(&repo);
                if result.is_ok() {
                    OutputFormatter::display(OutputType::Success, "Status: \n".to_string());

                    OutputFormatter::display(OutputType::Process, format!("{}", result.unwrap()));

                    OutputFormatter::display(
                        OutputType::Success,
                        format!("Checked status of repository: '{}'", repo),
                    );
                } else {
                    OutputFormatter::display(
                        OutputType::Error,
                        format!(
                            "Failed to check status in '{}': {}",
                            repo,
                            result.unwrap_err()
                        ),
                    );
                }
            }
            ValidCommand::Heads { repo } => {
                OutputFormatter::display(
                    OutputType::Process,
                    format!("Checking heads of repository: '{}'", repo),
                );
                let result = heads(&repo);
                if result.is_ok() {
                    OutputFormatter::display(OutputType::Success, "Heads: \n".to_string());

                    OutputFormatter::display(OutputType::Process, format!("{}", result.unwrap()));

                    OutputFormatter::display(
                        OutputType::Success,
                        format!("Checked heads of repository: '{}'", repo),
                    );
                } else {
                    OutputFormatter::display(
                        OutputType::Error,
                        format!(
                            "Failed to check heads in '{}': {}",
                            repo,
                            result.unwrap_err()
                        ),
                    );
                }
            }
            ValidCommand::Diff {
                branch_or_commit_1: commit_1,
                branch_or_commit_2: commit_2,
            } => {
                let id_1 = if commit_1.is_empty() {
                    "local HEAD"
                } else {
                    &commit_1
                };
                let id_2 = if commit_2 == REMOTE {
                    "remote HEAD"
                } else {
                    &commit_2
                };
                OutputFormatter::display(
                    OutputType::Process,
                    format!(
                        "Checking diff between branches or revisions '{}' and '{}'",
                        id_1, id_2
                    ),
                );
                let result = diff(".", &commit_1, &commit_2);
                if result.is_ok() {
                    OutputFormatter::display(OutputType::Success, "Changes: \n".to_string());

                    OutputFormatter::display(OutputType::Process, format!("{}", result.unwrap()));

                    OutputFormatter::display(
                        OutputType::Success,
                        format!(
                            "Checked diff between branches or revisions '{}' and '{}'",
                            id_1, id_2
                        ),
                    );
                } else {
                    OutputFormatter::display(
                        OutputType::Error,
                        format!(
                            "Failed to check diff between branches or revisions '{}' and '{}': {}",
                            id_1,
                            id_2,
                            result.unwrap_err()
                        ),
                    );
                }
            }
            ValidCommand::Cat { commit, path } => {
                OutputFormatter::display(
                    OutputType::Process,
                    format!("Displaying contents of file: '{}'", path),
                );
                let result = cat(".", &commit, &path);
                if result.is_ok() {
                    OutputFormatter::display(OutputType::Success, "Contents: \n".to_string());

                    OutputFormatter::display(OutputType::Process, format!("{}", result.unwrap()));

                    OutputFormatter::display(
                        OutputType::Success,
                        format!("Displayed contents of file: '{}'", path),
                    );
                } else {
                    OutputFormatter::display(
                        OutputType::Error,
                        format!(
                            "Failed to display contents of file '{}': {}",
                            path,
                            result.unwrap_err()
                        ),
                    );
                }
            }
            ValidCommand::Checkout { branch_or_commit } => {
                OutputFormatter::display(
                    OutputType::Process,
                    format!("Checking out branch or commit: '{}'", branch_or_commit),
                );
                let result = checkout(".", &branch_or_commit);
                if result.is_ok() {
                    OutputFormatter::display(
                        OutputType::Success,
                        format!("Checked out branch or commit: '{}'", branch_or_commit),
                    );
                } else {
                    OutputFormatter::display(
                        OutputType::Error,
                        format!(
                            "Failed to check out '{}': {}",
                            branch_or_commit,
                            result.unwrap_err()
                        ),
                    );
                }
            }
            ValidCommand::Commit { msg } => {
                OutputFormatter::display(
                    OutputType::Process,
                    format!("Committing changes with message: '{}'", msg),
                );
                let result = commit(".", &msg);
                if result.is_ok() {
                    OutputFormatter::display(
                        OutputType::Success,
                        format!("Committed changes with message: '{}'", msg),
                    );
                } else {
                    OutputFormatter::display(
                        OutputType::Error,
                        format!("Failed to commit changes: {}", result.unwrap_err()),
                    );
                }
            }
            ValidCommand::Log { repo } => {
                OutputFormatter::display(OutputType::Process, "Displaying commit log".to_string());
                let result = log(&repo);
                if result.is_ok() {
                    OutputFormatter::display(OutputType::Success, "Log: \n".to_string());

                    OutputFormatter::display(OutputType::Process, format!("{}", result.unwrap()));

                    OutputFormatter::display(
                        OutputType::Success,
                        "Displayed commit log".to_string(),
                    );
                } else {
                    OutputFormatter::display(
                        OutputType::Error,
                        format!("Failed to display log: {}", result.unwrap_err()),
                    );
                }
            }
            ValidCommand::Merge {
                branch_or_revision_from,
                branch_or_revision_into,
                msg,
            } => {
                let id_1 = if branch_or_revision_from == REMOTE {
                    "remote HEAD"
                } else {
                    &branch_or_revision_from
                };
                let id_2 = if branch_or_revision_into.is_empty() {
                    "local HEAD"
                } else {
                    &branch_or_revision_into
                };
                OutputFormatter::display(
                    OutputType::Process,
                    format!("Merging branch from '{}' into '{}': '{}'", id_1, id_2, msg),
                );
                let result = merge(
                    ".",
                    &branch_or_revision_into,
                    &branch_or_revision_from,
                    &msg,
                );
                if let Ok(report) = result {
                    OutputFormatter::display(OutputType::Success, "Changes: \n".to_string());

                    OutputFormatter::display(OutputType::Process, report);

                    OutputFormatter::display(
                        OutputType::Success,
                        format!("Merged branch from '{}' into '{}': '{}'", id_1, id_2, msg),
                    );
                } else {
                    OutputFormatter::display(
                        OutputType::Error,
                        format!(
                            "Failed to merge branches from '{}' into '{}': {}",
                            id_1,
                            id_2,
                            result.unwrap_err()
                        ),
                    );
                }
            }
            ValidCommand::Pull {
                path,
                branch,
                all,
                force,
            } => {
                let branch_name = if branch.is_empty() {
                    "HEAD"
                } else {
                    &format!("'{}'", branch)
                };
                OutputFormatter::display(
                    OutputType::Process,
                    format!(
                        "Pulling changes from '{}' {} to local HEAD",
                        path, branch_name,
                    ),
                );
                let result = pull(".", &path, &branch, all, force);
                if let Ok(report) = result {
                    OutputFormatter::display(OutputType::Success, "Changes: \n".to_string());

                    OutputFormatter::display(OutputType::Process, report);

                    OutputFormatter::display(
                        OutputType::Success,
                        format!(
                            "Pulled changes from '{}' {} to local HEAD",
                            path, branch_name,
                        ),
                    );
                } else {
                    OutputFormatter::display(
                        OutputType::Error,
                        format!(
                            "Failed to pull changes from '{}' {} to local HEAD: {}",
                            path,
                            branch_name,
                            result.unwrap_err()
                        ),
                    );
                }
            }
            ValidCommand::Push {
                path,
                branch,
                all,
                force,
            } => {
                let branch_name = if branch.is_empty() {
                    "local HEAD"
                } else {
                    &format!("'{}'", branch)
                };
                OutputFormatter::display(
                    OutputType::Process,
                    format!("Pushing changes from {} to '{}'", branch_name, path,),
                );
                let result = push(".", &path, &branch, all, force);
                if let Ok(report) = result {
                    OutputFormatter::display(OutputType::Success, "Changes: \n".to_string());

                    OutputFormatter::display(OutputType::Process, report);

                    OutputFormatter::display(
                        OutputType::Success,
                        format!("Pushed changes from {} to '{}'", branch_name, path,),
                    );
                } else {
                    OutputFormatter::display(
                        OutputType::Error,
                        format!(
                            "Failed to push changes from {} to '{}': {}",
                            branch_name,
                            path,
                            result.unwrap_err()
                        ),
                    );
                }
            }
            ValidCommand::Branch { branch } => {
                OutputFormatter::display(
                    OutputType::Process,
                    format!("Creating branch: '{}'", branch),
                );
                let result = init_branch(".", &branch, false);
                if result.is_ok() {
                    OutputFormatter::display(
                        OutputType::Success,
                        format!("Created branch: '{}'", branch),
                    );
                } else {
                    OutputFormatter::display(
                        OutputType::Error,
                        format!("Failed to create branch: '{}'", result.unwrap_err()),
                    );
                }
            }
        }
    }
}
