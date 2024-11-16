// days/src/a_2_behavioral_hiding/b_2_3_output_formatter.rs
//
// B.2.3 Output Formatter
// This component is responsible for formatting output to the command line
// based on the success or failure of an operation. It clearly distinguishes
// between success and error messages with color coding.
// 
// Parent Module: A.2 Behavioral Hiding
//
// ## Features
// - Prints success messages in **green**.
// - Prints error messages in **red**.
// - Handles user feedback in a consistent, visually distinct way.
//
// ## Dependencies
// This module does not depend on external libraries, relying on ANSI escape codes
// for color formatting in compatible terminals.
// 
// ## Example Usage
// ```rust
// use a_2_behavioral_hiding::output_formatter::{OutputFormatter, OutputType};
//
// fn main() {
//     OutputFormatter::display(OutputType::Success, "Operation completed successfully!".to_string());
//     OutputFormatter::display(OutputType::Error, "An error occurred during the operation.".to_string());
// }

#[derive(Debug, PartialEq)]
pub enum OutputType {
    Success,
    Error,
}

pub struct OutputFormatter;

impl OutputFormatter {
    pub fn display(output_type: OutputType, message: String) {
        match output_type {
            OutputType::Success => {
                println!("\x1b[32m{}\x1b[0m", message);
            }
            OutputType::Error => {
                eprintln!("\x1b[31m{}\x1b[0m", message);
            }
        }
    }
}