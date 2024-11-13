// days/src/behavioral_hiding/command_parser.rs

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
    Checkout {revision: String},
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