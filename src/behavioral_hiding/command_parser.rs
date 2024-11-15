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

#[derive(Debug, PartialEq)]
pub enum ValidCommand {
    Init,
    Clone {remote_url: String},
    Add {file: String},
    Remove {file: String},
    Status,
    Heads,
    Diff {revision_1: String, revision_2: String},
    Cat {file: String},
    Checkout {branch: String},
    Commit {message: String},
    Log,
    Merge {branch: String},
    Pull,
    Push {branch: Option<String>},
}

pub fn parse_command(args: Vec<String>) -> Result<ValidCommand, String> {
    // let cur_dir = env::current_dir()
    //     .map_err(|e| format!("Failed to fetch current directory: {}", e))?;
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
                .num_args(1..)
            ),
        )
        .subcommand(
            Command::new("add")
            .about("Add a file to the staging area")
            .arg(
                Arg::new("file")
                .help("File to add to the staging area")
                .required(true)
                .num_args(1)
            ),
        )
        .subcommand(
            Command::new("remove")
            .about("Remove a file from the staging area")
            .arg(
                Arg::new("file")
                .help("File to remove from the staging area")
                .required(true)
                .num_args(1)
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
                .num_args(1)
            )
            .arg(
                Arg::new("revision_2")
                .help("The second revision to compare")
                .required(true)
                .num_args(1)
            ),
        )
        .subcommand(
            Command::new("cat")
            .about("Inspect the content of a file")
            .arg(
                Arg::new("file")
                .help("Path to the file to inspect")
                .required(true)
                .num_args(1)
            ),
        )
        .subcommand(
            Command::new("checkout")
            .about("Switch to a different branch")
            .arg(
                Arg::new("branch")
                .help("Branch to switch to")
                .required(true)
                .num_args(1)
            ),
        )
        .subcommand(
            Command::new("commit")
            .about("Commit the changes in the staging area")
            .arg(
                Arg::new("message")
                .help("Commit message")
                .required(true)
                .num_args(1)
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
                .num_args(1)
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
                .required(false) // defaults to main branch if no argument taken
                .num_args(1)
            ),
        )
        .try_get_matches_from(args.clone())
        .map_err(|e| e.to_string())?;

    match matches.subcommand() {
        Some(("init", _)) => Ok(ValidCommand::Init),
        Some(("clone", sub_m)) => parse_clone(sub_m),
        Some(("add", sub_m)) => parse_add(sub_m),
        Some(("remove", sub_m)) => parse_remove(sub_m),
        Some(("status", _)) => Ok(ValidCommand::Status),
        Some(("heads", _)) => Ok(ValidCommand::Heads),
        Some(("diff", sub_m)) => parse_diff(sub_m),
        Some(("cat", sub_m)) => parse_cat(sub_m),
        Some(("checkout", sub_m)) => parse_checkout(sub_m),
        Some(("commit", sub_m)) => parse_commit(sub_m),
        Some(("log", _)) => Ok(ValidCommand::Log),
        Some(("merge", sub_m)) => parse_merge(sub_m),
        Some(("pull", _)) => Ok(ValidCommand::Pull),
        Some(("push", sub_m)) => parse_push(sub_m),
        _ => Err("Invalid command".to_string()),
    }
}

fn parse_clone(matches: &ArgMatches) -> Result<ValidCommand, String> {
    let remote_url = matches.get_one::<String>("remote_url").unwrap().to_string();
    Ok(ValidCommand::Clone { remote_url })
}

fn parse_add(matches: &ArgMatches) -> Result<ValidCommand, String> {
    let file = matches.get_one::<String>("file").unwrap().to_string();
    Ok(ValidCommand::Add { file })
}

fn parse_remove(matches: &ArgMatches) -> Result<ValidCommand, String> {
    let file = matches.get_one::<String>("file").unwrap().to_string();
    Ok(ValidCommand::Remove { file })
}

fn parse_diff(matches: &ArgMatches) -> Result<ValidCommand, String> {
    let revision_1 = matches.get_one::<String>("revision_1").unwrap().to_string();
    let revision_2 = matches.get_one::<String>("revision_2").unwrap().to_string();
    Ok(ValidCommand::Diff { revision_1, revision_2 })
}

fn parse_cat(matches: &ArgMatches) -> Result<ValidCommand, String> {
    let file = matches.get_one::<String>("file").unwrap().to_string();
    Ok(ValidCommand::Cat { file })
}

fn parse_checkout(matches: &ArgMatches) -> Result<ValidCommand, String> {
    let branch = matches.get_one::<String>("branch").unwrap().to_string();
    Ok(ValidCommand::Checkout { branch })
}

fn parse_commit(matches: &ArgMatches) -> Result<ValidCommand, String> {
    let message = matches.get_one::<String>("message").unwrap().to_string();
    Ok(ValidCommand::Commit { message })
}

fn parse_merge(matches: &ArgMatches) -> Result<ValidCommand, String> {
    let branch = matches.get_one::<String>("branch").unwrap().to_string();
    Ok(ValidCommand::Merge { branch })
}

fn parse_push(matches: &ArgMatches) -> Result<ValidCommand, String> {
    let branch = matches.get_one::<String>("branch").map(|s| s.to_string());
    Ok(ValidCommand::Push { branch })
}