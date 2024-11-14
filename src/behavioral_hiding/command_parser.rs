// days/src/behavioral_hiding/command_parser.rs
//
// B.2.1 Command Parser
// This component is responsible for parsing and validating command-line arguments 
// and returning a structured command based on user input.
// 
// Parent Module: A.2 Behavioral Hiding 
//
// Usage:
// The `parse_command` function utilizes the `clap` crate to define and interpret various 
// subcommands and their associated arguments. Valid commands include repository 
// initialization, file staging, branching, and committing, among others.
// 
// Each subcommand is mapped to a `ValidCommand` enum variant, allowing other 
// modules within the system to handle parsed commands in a type-safe manner.
//
// Dependencies:
// - clap: For command-line argument parsing.
// 
// Author: Yifan (Alvin) Jiang
// Date: 11/13/2024

use clap::{Arg, Command, ArgMatches};
use std::env;

#[derive(Debug)]
pub enum ValidCommand {
    Init,
    Clone {remote_url: String},
    Add {file: String},
    Remove {file: String},
    Status {directory: String},
    Heads,
    Diff (revision_1: String, revision_2: String),
    Cat {file: String},
    Checkout {branch: String},
    Commit {message: String},
    Log,
    Merge {branch: String},
    Pull,
    Push {branch: String},
}

pub fn parse_command() -> ValidCommand {
    let args: Vec<String> = env::args().skip(1).collect();
    let cur_dir = env::current_dir()
        .map_err(|e| format!("Failed to fetch current directory: {}", e))?;
    let matches = Command::new("days")
        .about("A distributed version control system developed in Rust")
        .subcommand(
            Command::new("init")
            .about("Initialize a new repository")
        )
        .subcommand(
            Command::new("clone")
            .about("Clone a repository")
            .arg(
                Arg::new("remote_url")
                .help("Remote URL of the repository to clone from")
                .required(true)
                .takes_value(true)
            ),
        )
        .subcommand(
            Command::new("add")
            .about("Add a file to the staging area")
            .arg(
                Arg::new("file")
                .help("File to add to the staging area")
                .required(true)
                .takes_value(true)
            ),
        )
        .subcommand(
            Command::new("remove")
            .about("Remove a file from the staging area")
            .arg(
                Arg::new("file")
                .help("File to remove from the staging area")
                .required(true)
                .takes_value(true)
            ),
        )
        .subcommand(
            Command::new("status")
            .about("Show the status of the repository")
        )
        .subcommand(
            Command::new("heads")
            .about("Show all branches in the repository")
        )
        .subcommand(
            Command::new("diff")
            .about("Show the differences between two revisions")
            .arg(
                Arg::new("revision_1")
                .help("The first revision to compare")
                .required(true)
                .takes_value(true)
            )
            .arg(
                Arg::new("revision_2")
                .help("The second revision to compare")
                .required(true)
                .takes_value(true)
            ),
        )
        .subcommand(
            Command::new("cat")
            .about("Inspect the content of a file")
            .arg(
                Arg::new("file")
                .help("Path to the file to inspect")
                .required(true)
                .takes_value(true)
            ),
        )
        .subcommand(
            Command::new("checkout")
            .about("Switch to a different branch")
            .arg(
                Arg::new("branch")
                .help("Branch to switch to")
                .required(true)
                .takes_value(true)
            ),
        )
        .subcommand(
            Command::new("commit")
            .about("Commit the changes in the staging area")
            .arg(
                Arg::new("message")
                .help("Commit message")
                .required(true)
                .takes_value(true)
            ),
        )
        .subcommand(
            Command::new("log")
            .about("Displays the commit log of the repository")
        )
        .subcommand(
            Command::new("merge")
            .about("Merge a branch into the current branch")
            .arg(
                Arg::new("branch")
                .help("Branch to merge into the current branch")
                .required(true)
                .takes_value(true)
            ),
        )
        .subcommand(
            Command::new("pull").about("Pulls changes from a remote repository")
        )
        .subcommand(
            Command::new("push").about("Pushes changes to a remote repository")
            .arg(
                Arg::new("branch")
                .help("Branch to push to the remote repository")
                .required(false)
                .takes_value(true)
            ),
        )
        .try_get_matches_from(args.clone())
        .map_err(|e| e.to_string())?;

    match command.as_str() {
        "init" => ValidCommand::Init,
        "clone" => {
            let remote_url = args[2].clone();
            ValidCommand::Clone {remote_url}
        },
        "add" => {
            let file = args[2].clone();
            ValidCommand::Add {file}
        },
        "remove" => {
            let file = args[2].clone();
            ValidCommand::Remove {file}
        },
        "status" => {
            let directory = args[2].clone();
            ValidCommand::Status {directory}
        },
        "heads" => ValidCommand::Heads,
        "diff" => {
            let revision_1 = args[2].clone();
            let revision_2 = args[3].clone();
            ValidCommand::Diff (revision_1, revision_2)
        },
        "cat" => {
            let file = args[2].clone();
            ValidCommand::Cat {file}
        },
        "checkout" => {
            let revision = args[2].clone();
            ValidCommand::Checkout {revision}
        },
        "commit" => {
            let message = args[2].clone();
            ValidCommand::Commit {message}
        },
        "log" => ValidCommand::Log,
        "merge" => {
            let branch = args[2].clone();
            ValidCommand::Merge {branch}
        },
        "pull" => ValidCommand::Pull,
        "push" => {
            let branch = args[2].clone();
            ValidCommand::Push {branch}
        },
        _ => panic!("Invalid command"),
    }
}