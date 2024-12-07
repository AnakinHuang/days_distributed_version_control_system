// days_dvcs/src/a_2_behavioral_hiding/b_2_3_output_formatter.rs
//
//! B.2.3 Output Formatter
//! This component is responsible for formatting output to the command line
//! based on the success or failure of an operation. It clearly distinguishes
//! between success and error messages with color coding.
//!
//! Parent Module: A.2 Behavioral Hiding
//!
//! ## Features
//! - Prints success messages in **green**.
//! - Prints error messages in **red**.
//!
//! ## Dependencies
//! - This module does not depend on external libraries.
//! - It uses ANSI escape codes for color formatting in compatible terminals.
//!
//! Author: Yifan (Alvin) Jiang
//! Date: 11/16/2024

#[derive(Debug, PartialEq)]
pub enum OutputType {
    Success,
    Process,
    Error,
}

pub struct OutputFormatter;

impl OutputFormatter {
    pub fn display(output_type: OutputType, message: String) {
        let color_code = match output_type {
            OutputType::Success => "\x1b[32m", // Green
            OutputType::Process => "\x1b[33m", // Yellow
            OutputType::Error => "\x1b[31m",   // Red
        };

        if message.contains("\x1b[") {
            for line in message.lines() {
                println!("{}", line);
            }
        } else {
            for line in message.lines() {
                println!("{}{}\x1b[0m", color_code, line);
            }
        }
    }
}
