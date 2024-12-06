// days_dvcs/src/a_2_behavioral_hiding/b_2_1_command_parser.rs
//
//! B.2.1 Command Parser
//! This component is responsible for parsing and validating command-line arguments
//! and returning a structured command based on user input.
//!
//! Parent Module: A.2 Behavioral Hiding
//!
//! ## Usage:
//! The `parse_command()` function utilizes the `clap` crate to define and interpret various
//! subcommands and their associated arguments. Valid commands include repository
//! initialization, file staging, branching, and committing, among others.
//!
//! Each subcommand is mapped to a `ValidCommand` enum variant, allowing other
//! modules within the system to handle parsed commands in a type-safe manner.
//!
//! The `init`, `clone`, `merge`, `pull`, and `push` commands accept an optional `--dir` or `-d`
//! flag to specify the directory in which the operation should be performed. This flag defaults
//! to the current directory (i.e. `.`) if not provided.
//!
//! Additionally, the `push` command accepts an optional `branch` argument to specify the branch
//! to push to. If not provided, the default branch is set to `main`.
//!
//! ## Dependencies:
//! - clap: For command-line argument parsing.
//!
//! Author: Yifan (Alvin) Jiang
//! Date: 11/13/2024

use crate::a_1_file_system_hiding::REMOTE;

use clap::{arg, ArgMatches, Command};
use clap::error::ErrorKind::InvalidSubcommand;

#[derive(Debug, PartialEq)]
pub enum ValidCommand {
    Init { directory: String },
    Clone { repo: String, directory: String },
    Add { pathspec: Vec<String> },
    Remove { pathspec: Vec<String> },
    Status { repo: String },
    Heads { repo: String },
    Diff { commit_1: String, commit_2: String },
    Cat { commit: String, path: String },
    Checkout { branch_or_commit: String },
    Commit { msg: String },
    Log { repo: String },
    Merge { branch: String },
    Pull { path: String, branch: String, all: bool, force: bool },
    Push { path: String, branch: String, all: bool, force: bool },
    Branch { branch: String },
}

pub fn parse_command(args: Vec<String>) -> Result<ValidCommand, clap::Error> {
    let matches = Command::new("days_dvcs")
        .about("Group DAYS distributed version control system developed in Rust")
        .version("2.0")
        .author(
            "Yuesong Huang <yhu116@ur.rochester.edu>\n\
            Alvin Jiang <yjiang54@ur.rochester.edu>\n\
            Duy Pham <tuanduy601@gmail.com>\n\
            Shervin Tursun-Zade <s.tursun-zade@rochester.edu>",
        )
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("init")
                .about("Initialize a new repository")
                .arg(
                    arg!([directory] "Directory for initialize the repository").default_value("."),
                ),
        )
        .subcommand(
            Command::new("clone")
                .about("Clone a repository")
                .arg(arg!(<repo> "Directory of the repository to clone"))
                .arg(arg!([directory] "Directory to clone the repository into").default_value("."))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("add")
                .about("Add a file to the staging area")
                .arg(arg!(<pathspec>... "Files to add"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("remove")
                .about("Remove a file from the staging area")
                .arg(arg!(<pathspec>... "Files to remove"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("status")
                .about("Display the status of the repository")
                .arg(arg!([repo] "Directory of the repository").default_value(".")),
        )
        .subcommand(
            Command::new("heads")
                .about("Display the heads of the repository")
                .arg(arg!([repo] "Directory of the repository").default_value(".")),
        )
        .subcommand(
            Command::new("diff")
                .about("Display the difference between two revisions")
                .arg(arg!(base: <commit> "First revision ID"))
                .arg(arg!(head: <commit> "Second revision ID"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("cat")
                .about("Display the content of a file at a specific revision")
                .arg(arg!(<commit> "Revision ID"))
                .arg(arg!(<path> "File path"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("checkout")
                .about("Switch to a different branch or revision")
                .arg(arg!(<branch_or_commit> "Branch or revision id"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("commit")
                .about("Commit the staged changes")
                .arg(arg!([msg] "Commit message").default_value("N/A")),
        )
        .subcommand(
            Command::new("log")
                .about("Display the commit log of the repository")
                .arg(arg!([repo] "Directory of the repository").default_value(".")),
        )
        .subcommand(
            Command::new("merge")
                .about("Merge a branch into the current branch")
                .arg(arg!(<branch> "Branch to merge"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("pull")
                .about("Pull changes from another repository")
                .arg(arg!([PATH] "Directory of the remote repository").default_value(REMOTE))
                .arg(arg!([BRANCH] "Branch to pull"))
                .arg(arg!(--all "Pull all branches"))
                .arg(arg!(-f --force "Force overwrite of local branch")),
        )
        .subcommand(
            Command::new("push")
                .about("Push changes to another repository")
                .arg(arg!([PATH] "Directory of the repository to push to").default_value(REMOTE))
                .arg(arg!([BRANCH] "Branch to pull"))
                .arg(arg!(--all "Push all branches"))
                .arg(arg!(-f --force "Force overwrite of local branch")),
        )
        .subcommand(
            Command::new("branch")
                .about("Create a new branch")
                .arg(arg!(<branch> "Branch name"))
                .arg_required_else_help(true),
        )
        .get_matches_from(args);

    match matches.subcommand() {
        Some(("init", sub_m)) => parse_init(sub_m),
        Some(("clone", sub_m)) => parse_clone(sub_m),
        Some(("add", sub_m)) => parse_add(sub_m),
        Some(("remove", sub_m)) => parse_remove(sub_m),
        Some(("status", sub_m)) => parse_status(sub_m),
        Some(("heads", sub_m)) => parse_heads(sub_m),
        Some(("diff", sub_m)) => parse_diff(sub_m),
        Some(("cat", sub_m)) => parse_cat(sub_m),
        Some(("checkout", sub_m)) => parse_checkout(sub_m),
        Some(("commit", sub_m)) => parse_commit(sub_m),
        Some(("log", sub_m)) => parse_log(sub_m),
        Some(("merge", sub_m)) => parse_merge(sub_m),
        Some(("pull", sub_m)) => parse_pull(sub_m),
        Some(("push", sub_m)) => parse_push(sub_m),
        Some(("branch", sub_m)) => parse_branch(sub_m),
        _ => Err(clap::Error::new(InvalidSubcommand)),
    }
}

fn parse_init(matches: &ArgMatches) -> Result<ValidCommand, clap::Error> {
    let directory = matches.get_one::<String>("directory").unwrap().to_string();
    Ok(ValidCommand::Init { directory })
}

fn parse_clone(matches: &ArgMatches) -> Result<ValidCommand, clap::Error> {
    let repo = matches.get_one::<String>("repo").unwrap().to_string();
    let directory = matches.get_one::<String>("directory").unwrap().to_string();
    Ok(ValidCommand::Clone { repo, directory })
}

fn parse_add(matches: &ArgMatches) -> Result<ValidCommand, clap::Error> {
    let files: Vec<String> = matches
        .get_many::<String>("pathspec")
        .unwrap()
        .map(|s| s.to_string())
        .collect();
    Ok(ValidCommand::Add { pathspec: files })
}

fn parse_remove(matches: &ArgMatches) -> Result<ValidCommand, clap::Error> {
    let files: Vec<String> = matches
        .get_many::<String>("pathspec")
        .unwrap()
        .map(|s| s.to_string())
        .collect();
    Ok(ValidCommand::Remove { pathspec: files })
}

fn parse_status(matches: &ArgMatches) -> Result<ValidCommand, clap::Error> {
    let repo = matches.get_one::<String>("repo").unwrap().to_string();
    Ok(ValidCommand::Status { repo })
}

fn parse_heads(matches: &ArgMatches) -> Result<ValidCommand, clap::Error> {
    let repo = matches.get_one::<String>("repo").unwrap().to_string();
    Ok(ValidCommand::Heads { repo })
}

fn parse_diff(matches: &ArgMatches) -> Result<ValidCommand, clap::Error> {
    let commit_1 = matches.get_one::<String>("base").unwrap().to_string();
    let commit_2 = matches.get_one::<String>("head").unwrap().to_string();
    Ok(ValidCommand::Diff { commit_1, commit_2 })
}

fn parse_cat(matches: &ArgMatches) -> Result<ValidCommand, clap::Error> {
    let commit = matches.get_one::<String>("commit").unwrap().to_string();
    let path = matches.get_one::<String>("path").unwrap().to_string();
    Ok(ValidCommand::Cat { commit, path })
}

fn parse_checkout(matches: &ArgMatches) -> Result<ValidCommand, clap::Error> {
    let branch_or_commit = matches
        .get_one::<String>("branch_or_commit")
        .unwrap()
        .to_string();
    Ok(ValidCommand::Checkout { branch_or_commit })
}

fn parse_commit(matches: &ArgMatches) -> Result<ValidCommand, clap::Error> {
    let msg = matches.get_one::<String>("msg").unwrap().to_string();
    Ok(ValidCommand::Commit { msg })
}

fn parse_log(matches: &ArgMatches) -> Result<ValidCommand, clap::Error> {
    let repo = matches.get_one::<String>("repo").unwrap().to_string();
    Ok(ValidCommand::Log { repo })
}

fn parse_merge(matches: &ArgMatches) -> Result<ValidCommand, clap::Error> {
    let branch = matches.get_one::<String>("branch").unwrap().to_string();
    Ok(ValidCommand::Merge { branch })
}

fn parse_pull(matches: &ArgMatches) -> Result<ValidCommand, clap::Error> {
    let path = matches.get_one::<String>("PATH").unwrap().to_string();
    let branch = matches.get_one::<String>("BRANCH").unwrap_or(&String::new()).to_string();
    let all = matches.get_flag("all");
    let force = matches.get_flag("force");
    Ok(ValidCommand::Pull { path, branch, all, force })
}

fn parse_push(matches: &ArgMatches) -> Result<ValidCommand, clap::Error> {
    let path = matches.get_one::<String>("PATH").unwrap().to_string();
    let branch = matches.get_one::<String>("BRANCH").unwrap_or(&String::new()).to_string();
    let all = matches.get_flag("all");
    let force = matches.get_flag("force");
    Ok(ValidCommand::Push { path, branch, all, force })
}

fn parse_branch(matches: &ArgMatches) -> Result<ValidCommand, clap::Error> {
    let branch = matches.get_one::<String>("branch").unwrap().to_string();
    Ok(ValidCommand::Branch { branch })
}
