// days_dvcs/src/a_3_repository_hiding/b_3_3_branch_management.rs
//

use std::collections::HashMap;
use crate::a_1_file_system_hiding::b_1_1_file_interaction::{check_file, read_file, read_struct, write_struct, delete_file};
use crate::a_1_file_system_hiding::b_1_2_directory_interaction::create_directory;
use crate::a_3_repository_hiding::b_3_1_repository_management::load_repo_metadata;
use crate::a_3_repository_hiding::b_3_2_revision_management::load_revision_metadata;

use std::io;
use chrono::DateTime;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BranchMetadata {
    pub name: String,             // Branch name
    pub head_commit: Option<String>,      // Latest commit on the branch
    pub commits: Vec<String>, // Base commit (for merges or new branches)
    pub staging: Vec<String>,     // Files staged for commit on this branch
    pub working: Vec<String>,     // Files in the working directory
}

pub fn init_branch(path: &str, branch: &str) -> Result<(), std::io::Error> {
    let branch_path = format!("{}/.dvcs/origin/{}", path, branch);
    create_directory(&format!("{}/commits", branch_path))?;
    create_directory(&format!("{}/staging", branch_path))?;
    create_directory(&format!("{}/.metadata", branch_path))?;
    create_directory(&format!("{}/working", branch_path))?;

    let init_metadata = BranchMetadata {
        name: branch.to_string(),
        head_commit: None,
        commits: Vec::new(),
        staging: Vec::new(),
        working: Vec::new(),
    };

    save_branch_metadata(path, branch, &init_metadata)?;
    Ok(())
}

pub fn load_branch_metadata(path: &str, branch: &str) -> Result<BranchMetadata, std::io::Error> {
    let metadata_path = format!("{}/.dvcs/origin/{}/.metadata/metadata.json", path, branch);
    let metadata: BranchMetadata = read_struct(&metadata_path)?;
    Ok(metadata)
}

pub fn save_branch_metadata(path: &str, branch: &str, metadata: &BranchMetadata) -> Result<(), std::io::Error> {
    let metadata_path = format!("{}/.dvcs/origin/{}/.metadata/metadata.json", path, branch);
    write_struct(&metadata_path, metadata)?;
    Ok(())
}

pub fn heads(path: &str) -> Result<Vec<String>, io::Error> {
    let repo_metadata = load_repo_metadata(path)?;
    let branch_metadata = load_branch_metadata(path, &repo_metadata.head)?;
    let branch = branch_metadata.name;
    let mut heads_map = HashMap::new();

    for revision_id in branch_metadata.commits.iter() {
        let revision_metadata = load_revision_metadata(path, &branch, revision_id)?;
        let date_time: DateTime::<chrono::Local> = revision_metadata.timestamp.into();

        let content = format!(
            "commit {} (HEAD -> {}, {})\nDate: {}\n\n\t{}\n\n",
            revision_metadata.id,
            branch,
            format!("{}/{}", path, branch),
            format!("{}", date_time.format("%Y-%m-%d %H:%M:%S")),
            revision_metadata.message,
        );

        heads_map.insert(revision_metadata.timestamp, content);
    }

    let mut sorted_heads_map: Vec<_> = heads_map.into_iter().collect();
    sorted_heads_map.sort_by(|a, b| b.0.cmp(&a.0));
    let sorted_logs = sorted_heads_map.into_iter().map(|(_, content)| content).collect();

    Ok(sorted_logs)
}

pub fn status(path: &str) -> Result<String, io::Error> {
    let repo_metadata = load_repo_metadata(path)?;
    let current_branch = &repo_metadata.head;
    let branch_metadata = load_branch_metadata(path, current_branch)?;

    let mut status_report = String::new();
    status_report.push_str(&format!("Current branch: {}\n", current_branch));
    status_report.push_str(&format!(
        "HEAD commit: {}\n",
        branch_metadata
            .head_commit
            .clone()
            .unwrap_or_else(|| "None".to_string())
    ));
    status_report.push_str("Staged files:\n");
    for file in branch_metadata.staging.iter() {
        status_report.push_str(&format!("\t{}\n", file));
    }
    status_report.push_str("Working directory files:\n");
    for file in branch_metadata.working.iter() {
        status_report.push_str(&format!("\t{}\n", file));
    }

    Ok(status_report)
}

pub fn add(repo_path: &str, files_path: &str, files: Vec<String>) -> Result<(), io::Error> {
    let repo_metadata = load_repo_metadata(repo_path)?;
    let branch = &repo_metadata.head;

    let mut branch_metadata = load_branch_metadata(repo_path, branch)?;
    
    for file in files {
        let file_path = format!("{}/{}", files_path, file);
        println!("file_path: {}", file_path);
        if check_file(&file_path) {
            let staging_path = format!("{}/.dvcs/origin/{}/staging/{}", repo_path, branch, file);

            if branch_metadata.staging.contains(&file) {
                branch_metadata.staging.retain(|f| f != &file);
                delete_file(&staging_path)?;
            }

            let content = read_file(&file_path)?;
            write_struct(&staging_path, &content)?;
            branch_metadata.staging.push(file.to_string());
        } else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("File '{}' does not exist in the working directory.", file),
            ));
        }
    }

    save_branch_metadata(repo_path, branch, &branch_metadata)?;
    Ok(())
}

pub fn remove(path: &str, files: Vec<String>) -> Result<(), io::Error> {
    let repo_metadata = load_repo_metadata(path)?;
    let branch = &repo_metadata.head;
    let mut branch_metadata = load_branch_metadata(path, branch)?;

    for file in files.iter() {
        if !branch_metadata.staging.contains(file) {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("File '{}' is not staged for commit.", file),
            ));
        }
    }

    for file in files.iter() {
        let staging_path = format!("{}/.dvcs/origin/{}/staging/{}", path, branch, file);

        if !check_file(&staging_path) {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("File '{}' does not exist in the staging directory.", file),
            ));
        }
    }

    for file in files {
        let staging_path = format!("{}/.dvcs/origin/{}/staging/{}", path, branch, file);
        delete_file(&staging_path)?;
        branch_metadata.staging.retain(|f| f != &file);
    }

    save_branch_metadata(path, branch, &branch_metadata)?;
    Ok(())
}

