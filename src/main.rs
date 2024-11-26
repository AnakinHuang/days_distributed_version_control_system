// days_dvcs/src/main.rs

mod a_1_file_system_hiding;
mod a_2_behavioral_hiding;
mod a_3_repository_hiding;

use a_2_behavioral_hiding::b_2_1_command_parser::{parse_command};
use a_2_behavioral_hiding::b_2_2_command_handler::CommandHandler;
use a_2_behavioral_hiding::b_2_3_output_formatter::{OutputFormatter, OutputType};


#[allow(dead_code)]
fn main() {
    let args: Vec<String> = std::env::args().collect();
    match parse_command(args) {
        Ok(command) => {
            match CommandHandler::handle_command(command) {
                Ok(_) => {
                    // OutputFormatter::display(OutputType::Success, "Command executed successfully!".to_string());
                }
                Err(err) => {
                    OutputFormatter::display(OutputType::Error, format!("Command failed: {}", err));
                }
            }
        }
        Err(err) => {
            OutputFormatter::display(OutputType::Error, format!("Failed to parse command: {}", err));
        }
    }
}