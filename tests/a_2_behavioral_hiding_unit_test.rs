// days_dvcs/tests/a_2_behavioral_hiding_unit_test.rs
//
// Unit tests for A.2 Behavioral Hiding Module
// To run these tests, use: cargo test --test a_2_behavioral_hiding_unit_test
//
// Author: Anakin (Yuesong Huang), Yifan (Alvin) Jiang
// Date: 11/14/2024

use days_dvcs::a_2_behavioral_hiding::b_2_1_command_parser::*;
use days_dvcs::a_2_behavioral_hiding::b_2_2_command_handler::*;
use days_dvcs::a_2_behavioral_hiding::b_2_3_output_formatter::*;

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;

    // B.2.1 Command Parser: test parse_command() function

    #[test]
    fn test_parse_init() {
        let args = vec!["days".to_string(), "init".to_string()];
        let command = parse_command(args).unwrap();
        assert_eq!(command, ValidCommand::Init {directory: Some(".".to_string())});
    }

    #[test]
    fn test_parse_init_with_directory() {
        let args = vec![
            "days".to_string(),
            "init".to_string(),
            "-d".to_string(),
            "/path/to/repo".to_string(),
        ];
        let command = parse_command(args).unwrap();
        assert_eq!(
            command,
            ValidCommand::Init {
                directory: Some("/path/to/repo".to_string())
            }
        );
    }

    #[test]
    fn test_parse_clone() {
        let args = vec!["days".to_string(), "clone".to_string(), "https://example.com/repo.git".to_string()];
        let command = parse_command(args).unwrap();
        assert_eq!(
            command,
            ValidCommand::Clone {
                remote_url: "https://example.com/repo.git".to_string(),
                directory: Some(".".to_string())
            }
        );
    }

    #[test]
    fn test_parse_clone_with_directory() {
        let args = vec![
            "days".to_string(),
            "clone".to_string(),
            "https://example.com/repo.git".to_string(),
            "--dir".to_string(),
            "/path/to/clone".to_string(),
        ];
        let command = parse_command(args).unwrap();
        assert_eq!(
            command,
            ValidCommand::Clone {
                remote_url: "https://example.com/repo.git".to_string(),
                directory: Some("/path/to/clone".to_string())
            }
        );
    }

    #[test]
    fn test_parse_add() {
        let args = vec!["days".to_string(), "add".to_string(), "file.txt".to_string()];
        let command = parse_command(args).unwrap();
        assert_eq!(
            command,
            ValidCommand::Add {
                file: "file.txt".to_string()
            }
        );
    }

    #[test]
    fn test_parse_remove() {
        let args = vec!["days".to_string(), "remove".to_string(), "file.txt".to_string()];
        let command = parse_command(args).unwrap();
        assert_eq!(
            command,
            ValidCommand::Remove {
                file: "file.txt".to_string()
            }
        );
    }

    #[test]
    fn test_parse_status() {
        let args = vec!["days".to_string(), "status".to_string()];
        let command = parse_command(args).unwrap();
        assert_eq!(command, ValidCommand::Status);
    }

    #[test]
    fn test_parse_heads() {
        let args = vec!["days".to_string(), "heads".to_string()];
        let command = parse_command(args).unwrap();
        assert_eq!(command, ValidCommand::Heads);
    }

    #[test]
    fn test_parse_diff() {
        let args = vec!["days".to_string(), "diff".to_string(), "rev1".to_string(), "rev2".to_string()];
        let command = parse_command(args).unwrap();
        assert_eq!(
            command,
            ValidCommand::Diff {
                revision_1: "rev1".to_string(),
                revision_2: "rev2".to_string()
            }
        );
    }

    #[test]
    fn test_parse_cat() {
        let args = vec!["days".to_string(), "cat".to_string(), "test_repo".to_string(), "123".to_string(), "file.txt".to_string()];
        let command = parse_command(args).unwrap();
        assert_eq!(
            command,
            ValidCommand::Cat {
                directory: Some("test_repo".to_string()),
                revision: "123".to_string(),
                file: "file.txt".to_string()
            }
        );
    }

    #[test]
    fn test_parse_checkout() {
        let args = vec!["days".to_string(), "checkout".to_string(), "branch".to_string()];
        let command = parse_command(args).unwrap();
        assert_eq!(
            command,
            ValidCommand::Checkout {
                branch: "branch".to_string()
            }
        );
    }

    #[test]
    fn test_parse_commit() {
        let args = vec!["days".to_string(), "commit".to_string(), "test_repo".to_string(), "message".to_string()];
        let command = parse_command(args).unwrap();
        assert_eq!(
            command,
            ValidCommand::Commit {
                directory: Some("test_repo".to_string()),
                message: "message".to_string()
            }
        );
    }

    #[test]
    fn test_parse_log() {
        let args = vec!["days".to_string(), "log".to_string()];
        let command = parse_command(args).unwrap();
        assert_eq!(command, ValidCommand::Log);
    }

    #[test]
    fn test_parse_merge() {
        let args = vec!["days".to_string(), "merge".to_string(), "branch".to_string()];
        let command = parse_command(args).unwrap();
        assert_eq!(
            command,
            ValidCommand::Merge {
                branch: "branch".to_string(),
                directory: Some(".".to_string())
            }
        );
    }

    #[test]
    fn test_parse_merge_with_directory() {
        let args = vec![
            "days".to_string(),
            "merge".to_string(),
            "feature-branch".to_string(),
            "-d".to_string(),
            "/path/to/repo".to_string(),
        ];
        let command = parse_command(args).unwrap();
        assert_eq!(
            command,
            ValidCommand::Merge {
                branch: "feature-branch".to_string(),
                directory: Some("/path/to/repo".to_string())
            }
        );
    }

    #[test]
    fn test_parse_pull() {
        let args = vec!["days".to_string(), "pull".to_string()];
        let command = parse_command(args).unwrap();
        assert_eq!(command, ValidCommand::Pull {directory: Some(".".to_string())});
    }

    #[test]
    fn test_parse_pull_with_directory() {
        let args = vec![
            "days".to_string(),
            "pull".to_string(),
            "--dir".to_string(),
            "/path/to/repo".to_string(),
        ];
        let command = parse_command(args).unwrap();
        assert_eq!(
            command,
            ValidCommand::Pull {
                directory: Some("/path/to/repo".to_string())
            }
        );
    }

    #[test]
    fn test_parse_push() {
        let args = vec!["days".to_string(), "push".to_string(), "branch".to_string()];
        let command = parse_command(args).unwrap();
        assert_eq!(
            command,
            ValidCommand::Push {
                branch: Some("branch".to_string()),
                directory: Some(".".to_string())
            }
        );
    }

    #[test]
    fn test_parse_push_no_branch_arg() {
        let args = vec!["days".to_string(), "push".to_string()];
        let command = parse_command(args).unwrap();
        assert_eq!(
            command,
            ValidCommand::Push {
                branch: Some("main".to_string()),
                directory: Some(".".to_string())
            }
        );
    }

    #[test]
    fn test_parse_push_with_directory() {
        let args = vec![
            "days".to_string(),
            "push".to_string(),
            "branch".to_string(),
            "-d".to_string(),
            "/path/to/repo".to_string(),
        ];
        let command = parse_command(args).unwrap();
        assert_eq!(
            command,
            ValidCommand::Push {
                branch: Some("branch".to_string()),
                directory: Some("/path/to/repo".to_string())
            }
        );
    }

    #[test]
    fn test_parse_invalid() {
        let args = vec!["days".to_string(), "invalid".to_string()];
        let command = parse_command(args);
        assert!(command.is_err());
    }

    // B.2.3 Output Formatter: test OutputFormatter::display() function
    // Run with `cargo test -- --nocapture` to make colored output visible

    #[test]
    fn test_output_success() {
        OutputFormatter::display(OutputType::Success, "succeeded!".to_string());
    }

    #[test]
    fn test_output_error() {
        OutputFormatter::display(OutputType::Error, "Expected error when failed!".to_string());
    }
}
