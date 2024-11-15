// days/tests/behavioral_hiding_test.rs
//
// Unit tests for A.2 Behavioral Hiding

use days_dvcs::behavioral_hiding::command_parser::{parse_command, ValidCommand};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_init_command() {
        let args = vec!["days".to_string(), "init".to_string()];
        let command = parse_command(args).unwrap();
        assert_eq!(command, ValidCommand::Init);
    }

    #[test]
    fn test_parse_clone_command() {
        let args = vec!["days".to_string(), "clone".to_string(), "https://example.com/repo.git".to_string()];
        let command = parse_command(args).unwrap();
        assert_eq!(
            command,
            ValidCommand::Clone {
                remote_url: "https://example.com/repo.git".to_string()
            }
        );
    }

    #[test]
    fn test_parse_add_command() {
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
    fn test_parse_remove_command() {
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
    fn test_parse_status_command() {
        let args = vec!["days".to_string(), "status".to_string()];
        let command = parse_command(args).unwrap();
        assert_eq!(command, ValidCommand::Status);
    }

    #[test]
    fn test_parse_heads_command() {
        let args = vec!["days".to_string(), "heads".to_string()];
        let command = parse_command(args).unwrap();
        assert_eq!(command, ValidCommand::Heads);
    }

    #[test]
    fn test_parse_diff_command() {
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
}
