// days_dvcs/tests/a_2_behavioral_hiding_unit_test.rs
//
// Unit tests for A.2 Behavioral Hiding Module
// To run these tests, use: cargo beta_tests --beta_tests a_2_behavioral_hiding_unit_test
//
// Author: Anakin (Yuesong Huang), Yifan (Alvin) Jiang
// Date: 11/14/2024

use days_dvcs::a_2_behavioral_hiding::b_2_1_command_parser::*;
use days_dvcs::a_2_behavioral_hiding::b_2_3_output_formatter::*;

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;

    /// B.2.1 Command Parser: beta_tests parse_command() function

    #[test]
    fn test_parse_init() {
        let args = vec!["days_dvcs".to_string(), "init".to_string()];
        let command = parse_command(args).unwrap();
        assert_eq!(
            command,
            ValidCommand::Init {
                directory: ".".to_string()
            }
        );
    }

    #[test]
    fn test_parse_init_with_directory() {
        let args = vec![
            "days_dvcs".to_string(),
            "init".to_string(),
            "/path/to/repo".to_string(),
        ];
        let command = parse_command(args).unwrap();
        assert_eq!(
            command,
            ValidCommand::Init {
                directory: "/path/to/repo".to_string()
            }
        );
    }

    #[test]
    fn test_parse_clone() {
        let args = vec![
            "days_dvcs".to_string(),
            "clone".to_string(),
            "https://example.com/repo.git".to_string(),
        ];
        let command = parse_command(args).unwrap();
        assert_eq!(
            command,
            ValidCommand::Clone {
                repo: "https://example.com/repo.git".to_string(),
                directory: ".".to_string()
            }
        );
    }

    #[test]
    fn test_parse_clone_with_directory() {
        let args = vec![
            "days_dvcs".to_string(),
            "clone".to_string(),
            ".remote".to_string(),
            "/path/to/clone".to_string(),
        ];
        let command = parse_command(args).unwrap();
        assert_eq!(
            command,
            ValidCommand::Clone {
                repo: ".remote".to_string(),
                directory: "/path/to/clone".to_string()
            }
        );
    }

    #[test]
    fn test_parse_add() {
        let args = vec![
            "days_dvcs".to_string(),
            "add".to_string(),
            "file.txt".to_string(),
        ];
        let command = parse_command(args).unwrap();
        assert_eq!(
            command,
            ValidCommand::Add {
                pathspec: vec!["file.txt".to_string()]
            }
        );
    }

    #[test]
    fn test_parse_remove() {
        let args = vec![
            "days_dvcs".to_string(),
            "remove".to_string(),
            "file.txt".to_string(),
        ];
        let command = parse_command(args).unwrap();
        assert_eq!(
            command,
            ValidCommand::Remove {
                pathspec: vec!["file.txt".to_string()]
            }
        );
    }

    #[test]
    fn test_parse_status() {
        let args = vec!["days_dvcs".to_string(), "status".to_string()];
        let command = parse_command(args).unwrap();
        assert_eq!(
            command,
            ValidCommand::Status {
                repo: ".".to_string()
            }
        );
    }

    #[test]
    fn test_parse_heads() {
        let args = vec!["days_dvcs".to_string(), "heads".to_string()];
        let command = parse_command(args).unwrap();
        assert_eq!(
            command,
            ValidCommand::Heads {
                repo: ".".to_string()
            }
        );
    }

    #[test]
    fn test_parse_diff() {
        let args = vec![
            "days_dvcs".to_string(),
            "diff".to_string(),
            "rev1".to_string(),
            "rev2".to_string(),
        ];
        let command = parse_command(args).unwrap();
        assert_eq!(
            command,
            ValidCommand::Diff {
                commit_1: "rev1".to_string(),
                commit_2: "rev2".to_string()
            }
        );
    }

    #[test]
    fn test_parse_cat() {
        let args = vec![
            "days_dvcs".to_string(),
            "cat".to_string(),
            "123".to_string(),
            "file.txt".to_string(),
        ];
        let command = parse_command(args).unwrap();
        assert_eq!(
            command,
            ValidCommand::Cat {
                commit: "123".to_string(),
                path: "file.txt".to_string(),
            }
        );
    }

    #[test]
    fn test_parse_checkout() {
        let args = vec![
            "days_dvcs".to_string(),
            "checkout".to_string(),
            "branch".to_string(),
        ];
        let command = parse_command(args).unwrap();
        assert_eq!(
            command,
            ValidCommand::Checkout {
                branch_or_commit: "branch".to_string(),
            }
        );
    }

    #[test]
    fn test_parse_commit() {
        let args = vec![
            "days_dvcs".to_string(),
            "commit".to_string(),
            "message".to_string(),
        ];
        let command = parse_command(args).unwrap();
        assert_eq!(
            command,
            ValidCommand::Commit {
                msg: "message".to_string(),
            }
        );
    }

    #[test]
    fn test_parse_log() {
        let args = vec!["days_dvcs".to_string(), "log".to_string()];
        let command = parse_command(args).unwrap();
        assert_eq!(
            command,
            ValidCommand::Log {
                repo: ".".to_string()
            }
        );
    }

    #[test]
    fn test_parse_merge() {
        let args = vec![
            "days_dvcs".to_string(),
            "merge".to_string(),
            "branch".to_string(),
        ];
        let command = parse_command(args).unwrap();
        assert_eq!(
            command,
            ValidCommand::Merge {
                branch: "branch".to_string(),
            }
        );
    }

    #[test]
    fn test_parse_merge_with_directory() {
        let args = vec![
            "days_dvcs".to_string(),
            "merge".to_string(),
            "feature-branch".to_string(),
        ];
        let command = parse_command(args).unwrap();
        assert_eq!(
            command,
            ValidCommand::Merge {
                branch: "feature-branch".to_string(),
            }
        );
    }

    #[test]
    fn test_parse_pull() {
        let args = vec!["days_dvcs".to_string(), "pull".to_string()];
        let command = parse_command(args).unwrap();
        assert_eq!(
            command,
            ValidCommand::Pull {
                path: ".remote".to_string(),
                branch: String::new(),
                all: false,
                force: false,
            }
        );
    }

    #[test]
    fn test_parse_pull_with_directory() {
        let args = vec![
            "days_dvcs".to_string(),
            "pull".to_string(),
            "/path/to/repo".to_string(),
        ];
        let command = parse_command(args).unwrap();
        assert_eq!(
            command,
            ValidCommand::Pull {
                path: "/path/to/repo".to_string(),
                branch: String::new(),
                all: false,
                force: false,
            }
        );
    }

    #[test]
    fn test_parse_push() {
        let args = vec![
            "days_dvcs".to_string(),
            "push".to_string(),
            "remote".to_string(),
            "branch".to_string(),
        ];
        let command = parse_command(args).unwrap();
        assert_eq!(
            command,
            ValidCommand::Push {
                path: "remote".to_string(),
                branch: "branch".to_string(),
                all: false,
                force: false,
            }
        );
    }

    #[test]
    fn test_parse_push_no_branch_arg() {
        let args = vec!["days_dvcs".to_string(), "push".to_string()];
        let command = parse_command(args).unwrap();
        assert_eq!(
            command,
            ValidCommand::Push {
                path: ".remote".to_string(),
                branch: String::new(),
                all: false,
                force: false,
            }
        );
    }

    #[test]
    fn test_parse_push_with_directory() {
        let args = vec![
            "days_dvcs".to_string(),
            "push".to_string(),
            "/path/to/repo".to_string(),
            "branch".to_string(),
        ];
        let command = parse_command(args).unwrap();
        assert_eq!(
            command,
            ValidCommand::Push {
                path: "/path/to/repo".to_string(),
                branch: "branch".to_string(),
                all: false,
                force: false,
            }
        );
    }

    /// B.2.3 Output Formatter: beta_tests OutputFormatter::display() function
    /// Run with `cargo beta_tests -- --nocapture` to make colored output visible

    #[test]
    fn test_output_success() {
        OutputFormatter::display(OutputType::Success, "Expect succeed!".to_string());
    }

    #[test]
    fn test_output_error() {
        OutputFormatter::display(OutputType::Error, "Expect error!".to_string());
    }
}
