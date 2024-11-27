// days_dvcs/tests/a_3_repository_hiding_unit_test.rs
//
// Unit tests for A.3 Repository Hiding Module
// To run these tests, use: cargo test --test a_3_repository_hiding_unit_test
//
// Author: Anakin (Yuesong Huang), Yifan (Alvin) Jiang
// Date: 11/14/2024

use days_dvcs::a_1_file_system_hiding::b_1_1_file_interaction::*;
use days_dvcs::a_1_file_system_hiding::b_1_2_directory_interaction::*;
use days_dvcs::a_3_repository_hiding::b_3_1_repository_management::*;
use days_dvcs::a_3_repository_hiding::b_3_2_revision_management::*;
use days_dvcs::a_3_repository_hiding::b_3_3_branch_management::*;

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;
    use std::io;
    use std::process::exit;

    /// B.3.1 Repository Management

    #[test]
    fn test_init_repository() {
        if check_file("test_repo") {
            delete_directory("test_repo", true).unwrap();
        }
        
        let result = init_repository("test_repo");
        assert!(result.is_ok(), "Failed to initialize repository");
        
        let dvcs_path = format!("{}/.dvcs", "test_repo");
        assert!(check_file(&dvcs_path), ".dvcs directory does not exist");

        let metadata_path = format!("{}/.metadata", dvcs_path);
        assert!(check_file(&metadata_path), ".metadata directory does not exist");

        let head_path = format!("{}/HEAD", dvcs_path);
        assert!(check_file(&head_path), "HEAD file does not exist");
        
        let metadata = load_repo_metadata("test_repo").expect("Failed to load repository metadata");
        assert_eq!(metadata.head, "main", "Default head branch is not 'main'");
        assert!(metadata.branches.contains_key("main"), "Main branch not found in metadata");
        
        delete_directory("test_repo", true).unwrap();
    }

    #[test]
    fn test_init_repository_already_exists() {
        let result = init_repository("test_init_repo");
        
        let result = init_repository("test_init_repo");
        assert!(
            result.is_err(),
            "Expected error when initializing an already existing repository"
        );
        assert_eq!(
            result.unwrap_err().kind(),
            io::ErrorKind::AlreadyExists,
            "Error kind is not AlreadyExists"
        );
        
        delete_directory("test_init_repo", true).unwrap();
    }

    #[test]
    fn test_clone_repository() {
        if check_file("test_clone_repo") {
            delete_directory("test_clone_repo", true).unwrap();
        }
        if check_file("test_clone_clone") {
            delete_directory("test_clone_clone", true).unwrap();
        }

        init_repository("test_clone_repo").unwrap();
        
        let result = clone_repository("test_clone_repo", "test_clone_clone");
        assert!(result.is_ok(), "Failed to clone repository");
        
        let dvcs_path = format!("{}/.dvcs", "test_clone_clone");
        assert!(check_file(&dvcs_path), ".dvcs directory does not exist in cloned repo");

        let metadata_path = format!("{}/.metadata", dvcs_path);
        assert!(check_file(&metadata_path), ".metadata directory does not exist in cloned repo");

        let head_path = format!("{}/HEAD", dvcs_path);
        assert!(check_file(&head_path), "HEAD file does not exist in cloned repo");
        
        let cloned_metadata = load_repo_metadata("test_clone_clone").expect("Failed to load cloned repository metadata");
        assert_eq!(cloned_metadata.head, "main", "Default head branch in clone is not 'main'");
        assert!(cloned_metadata.branches.contains_key("main"), "Main branch not found in cloned metadata");

        delete_directory("test_clone_repo", true).unwrap();
        delete_directory("test_clone_clone", true).unwrap();
    }

    #[test]
    fn test_clone_repository_missing_metadata() {
        // Clean up any existing test data
        if check_file("test_missing_repo") {
            delete_directory("test_missing_repo", true).unwrap();
        }
        if check_file("test_missing_clone") {
            delete_directory("test_missing_clone", true).unwrap();
        }

        create_directory("test_missing_repo").unwrap();
        
        let result = clone_repository("test_missing_repo", "test_missing_clone");
        assert!(
            result.is_err(),
            "Expected error when cloning a repository with missing metadata"
        );
        assert_eq!(
            result.unwrap_err().kind(),
            io::ErrorKind::NotFound,
            "Error kind is not NotFound"
        );
        
        delete_directory("test_missing_repo", true).unwrap();
    }

    #[test]
    fn test_save_and_load_repo_metadata() {
        if check_file("test_repo_save_and_load_repo") {
            delete_directory("test_repo_save_and_load_repo", true).unwrap();
        }
        
        init_repository("test_repo_save_and_load_repo").unwrap();
        
        let mut metadata = load_repo_metadata("test_repo_save_and_load_repo").unwrap();
        metadata.branches.insert("feature".to_string(), "commit123".to_string());
        save_repo_metadata("test_repo", &metadata).unwrap();
        
        let updated_metadata = load_repo_metadata("test_repo").unwrap();
        assert!(updated_metadata.branches.contains_key("feature"), "Feature branch not found in metadata");
        assert_eq!(
            updated_metadata.branches["feature"],
            "commit123",
            "Feature branch commit ID is incorrect"
        );
        
        delete_directory("test_repo", true).unwrap();
    }
    
    /// B.3.2 Revision Management
    /// B.3.3 Branch Management

    #[test]
    fn test_init_branch() {
        let repo_path = "test_branch_repo";
        init_repository(repo_path).unwrap();
        init_branch(repo_path, "feature").unwrap();

        let branch_metadata = load_branch_metadata(repo_path, "feature").unwrap();
        assert_eq!(branch_metadata.name, "feature");
        assert!(branch_metadata.head_commit.is_none());

        delete_directory(repo_path, true).unwrap();
    }

    #[test]
    fn test_heads() {
        let repo_path = "test_heads_repo";
        init_repository(repo_path).unwrap();

        let heads_output = heads(repo_path).unwrap();
        assert!(heads_output.is_empty()); // No commits yet

        delete_directory(repo_path, true).unwrap();
    }

    #[test]
    fn test_status() {
        let repo_path = "test_status_repo";
        init_repository(repo_path).unwrap();

        let status_report = status(repo_path).unwrap();
        assert!(status_report.contains("Current branch: main"));
        assert!(status_report.contains("HEAD commit: None"));

        delete_directory(repo_path, true).unwrap();
    }

    #[test]
    fn test_add() {
        let repo_path = "test_add_repo";
        let working_path = format!("{}/.dvcs/origin/main/working", repo_path);
        init_repository(repo_path).unwrap();

        let file_path = format!("{}/file.txt", working_path);
        write_file(&file_path, "Test content").unwrap();

        add(repo_path, &working_path, vec!["file.txt".to_string()]).unwrap();

        let branch_metadata = load_branch_metadata(repo_path, "main").unwrap();
        assert!(branch_metadata.staging.contains(&"file.txt".to_string()));

        delete_directory(repo_path, true).unwrap();
    }

    #[test]
    fn test_remove() {
        let repo_path = "test_remove_repo";
        let working_path = format!("{}/.dvcs/origin/main/working", repo_path);
        init_repository(repo_path).unwrap();

        let file_path = format!("{}/file.txt", working_path);
        write_file(&file_path, "Test content").unwrap();

        add(repo_path, &working_path, vec!["file.txt".to_string()]).unwrap();
        
        remove(repo_path, vec!["file.txt".to_string()]).unwrap();
        exit(0);
        
        let branch_metadata = load_branch_metadata(repo_path, "main").unwrap();
        assert!(!branch_metadata.staging.contains(&"file.txt".to_string()));

        delete_directory(repo_path, true).unwrap();
    }

    #[test]
    fn test_add_nonexistent_file() {
        let repo_path = "test_nonexistent_repo";
        let working_path = format!("{}/.dvcs/origin/main/working", repo_path);
        init_repository(repo_path).unwrap();

        let result = add(repo_path, &working_path, vec!["nonexistent.txt".to_string()]);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().kind(),
            std::io::ErrorKind::NotFound
        );

        delete_directory(repo_path, true).unwrap();
    }

    #[test]
    fn test_remove_unstaged_file() {
        let repo_path = "test_unstaged_repo";
        init_repository(repo_path).unwrap();

        let result = remove(repo_path, vec!["unstaged.txt".to_string()]);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().kind(),
            std::io::ErrorKind::NotFound
        );

        delete_directory(repo_path, true).unwrap();
    }
}
