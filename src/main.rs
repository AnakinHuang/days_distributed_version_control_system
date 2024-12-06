// days_dvcs/src/main.rs

mod a_1_file_system_hiding;
mod a_2_behavioral_hiding;
mod a_3_repository_hiding;

use std::env::args;
use a_2_behavioral_hiding::b_2_1_command_parser::{parse_command};
use a_2_behavioral_hiding::b_2_2_command_handler::{CommandHandler};

fn main() {
    let args: Vec<String> = args().collect();
    CommandHandler::handle_command(parse_command(args).unwrap());
}
