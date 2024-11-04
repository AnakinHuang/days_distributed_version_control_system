mod behavior_hiding;
mod file_system_hiding;
mod repository_hiding;

use behavior_hiding::BehaviorHiding;
use clap::{Arg, Command};
use std::process;

#[allow(dead_code)]
fn main() {
    let matches = Command::new("DAYS DVCS First Prototyping and Demo")
        .version("0.1")
        .author("Yuesong Huang <yhu116@ur.rochester.edu>\nAlvin Jiang <yjiang54@ur.rochester.edu>\nDuy Pham <tuanduy601@gmail.com>\nShervin Tursun-Zade <s.tursun-zade@rochester.edu>")
        .arg(
            Arg::new("command")
                .help("The DVCS command to execute")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("directory")
                .help("The directory to execute the command in")
                .required(true)
                .index(2),
        )
        .get_matches();

    let command = matches.get_one::<String>("command").unwrap();
    let directory = matches.get_one::<String>("directory").unwrap();

    match BehaviorHiding::validate_command(command) {
        Ok(valid_command) => {
            if let Err(e) = BehaviorHiding::execute_command(valid_command, directory) {
                BehaviorHiding::display_output(&format!("Error: {:?}", e), "error");
                process::exit(1);
            } else {
                BehaviorHiding::display_output("Command executed successfully", "success");
            }
        }
        Err(e) => {
            BehaviorHiding::display_output(&format!("Error: {:?}", e), "error");
            process::exit(1);
        }
    }
}
