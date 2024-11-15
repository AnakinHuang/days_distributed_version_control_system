mod behavior_hiding;
mod file_system_hiding;
mod repository_hiding;

use behavior_hiding::BehaviorHiding;
use clap::{Arg, Command};
use std::process;

fn main() {
    let matches = Command::new("DAYS DVCS First Prototyping and Demo")
        .version("0.1")
        .author("Yuesong Huang <yhu116@ur.rochester.edu>\nAlvin Jiang <yjiang54@ur.rochester.edu>\nDuy Pham <tuanduy601@gmail.com>\nShervin Tursun-Zade <s.tursun-zade@rochester.edu>")
        .about("Prototype for a Distributed Version Control System (DVCS)")
        .arg(
            Arg::new("command")
                .help("The DVCS command to execute (e.g., 'init', 'commit', 'checkout', 'status')")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("directory")
                .help("The repository directory to execute the command in")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::new("file")
                .help("The file path to commit (required for the 'commit' command)")
                .required(false)
                .index(3),
        )
        .get_matches();

    let command = matches.get_one::<String>("command").unwrap();
    let directory = matches.get_one::<String>("directory").unwrap();
    let file_path = matches.get_one::<String>("file");

    // Validate the command
    match BehaviorHiding::validate_command(command) {
        Ok(valid_command) => {
            // Execute the command with optional file path for specific commands
            if let Err(e) = BehaviorHiding::execute_command(
                valid_command,
                directory,
                file_path.map(|s| s.as_str()),
            ) {
                // Display error output and exit with status code 1 on failure
                BehaviorHiding::display_output(&format!("Error: {:?}", e), "error");
                process::exit(1);
            } else {
                // Display success message on successful command execution
                BehaviorHiding::display_output("Command executed successfully", "success");
            }
        }
        Err(e) => {
            // Display validation error message and exit
            BehaviorHiding::display_output(&format!("Error: {:?}", e), "error");
            process::exit(1);
        }
    }
}
