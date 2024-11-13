// days/src/behavioral_hiding/command_parser.rs

use clap::{Arg, Command, ArgMatches};
use std::env;

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