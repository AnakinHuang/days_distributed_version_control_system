// days_dvcs/src/a_3_repository_hiding/b_3_2_revision_management.rs
//

use crate::a_1_file_system_hiding::b_1_1_file_interaction::{delete_file, read_file, read_struct, write_file, write_struct};
use crate::a_1_file_system_hiding::b_1_2_directory_interaction::{create_directory, delete_directory};
use crate::a_3_repository_hiding::b_3_1_repository_management::{load_repo_metadata, save_repo_metadata};
use crate::a_3_repository_hiding::b_3_3_branch_management::{load_branch_metadata, save_branch_metadata};

use std::io;
use std::collections::HashMap;
use std::fmt::Debug;
use chrono::DateTime;
use std::time::SystemTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use md5;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RevisionMetadata {
    pub id: String,                    // Unique identifier for the revision
    pub files: HashMap<String, String>, // Maps file paths to their content hashes
    pub parents: Vec<String>,          // Parent revisions (supports merges)
    pub message: String,               // Commit message
    pub timestamp: SystemTime,             // Timestamp of the commit
}

pub fn load_revision_metadata(path: &str, branch: &str, revision_id: &str) -> Result<RevisionMetadata, io::Error> {
    let metadata_path = format!("{}/.dvcs/origin/{}/commits/{}/.metadata/metadata.json", path, branch, revision_id);
    let metadata: RevisionMetadata = read_struct(&metadata_path)?;
    Ok(metadata)
}

pub fn save_revision_metadata(path: &str, branch: &str, revision_id: &str, metadata: &RevisionMetadata) -> Result<(), io::Error> {
    let metadata_path = format!("{}/.dvcs/origin/{}/commits/{}/.metadata/metadata.json", path, branch, revision_id);
    write_struct(&metadata_path, metadata)?;
    Ok(())
}

pub fn commit(path: &str, message: &str) -> Result<String, io::Error> {
    let mut repo_metadata = load_repo_metadata(path)?;
    let mut branch_metadata = load_branch_metadata(path, &repo_metadata.head)?;

    if branch_metadata.staging.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "No changes to commit."));
    }

    let revision_id = Uuid::new_v4().to_string();
    let staged_path = format!("{}/.dvcs/origin/{}/staging", path, repo_metadata.head);
    let commit_path = format!("{}/.dvcs/origin/{}/commits/{}", path, repo_metadata.head, revision_id);
    
    create_directory(&commit_path)?;
    
    let mut files = HashMap::new();

    for file in branch_metadata.staging.iter() {
        let src_path = format!("{}/{}", staged_path, file);
        let dest_path = format!("{}/{}", commit_path, file);
        
        let content = read_file(&src_path)?;
        write_file(&dest_path, &content)?;
        let file_hash = format!("{:x}", md5::compute(content));
        files.insert(file.to_string(), file_hash);
        delete_file(&src_path)?;
    }
    
    let new_revision = RevisionMetadata {
        id: revision_id.clone(),
        files,
        parents: Vec::from([branch_metadata.head_commit.clone().unwrap_or_default()]),
        message: message.to_string(),
        timestamp: SystemTime::now(),
    };
    save_revision_metadata(path, &repo_metadata.head, &revision_id, &new_revision)?;
    
    branch_metadata.head_commit = Some(revision_id.clone());
    branch_metadata.commits.push(revision_id.clone());
    branch_metadata.staging.clear();
    save_branch_metadata(path, &repo_metadata.head, &branch_metadata)?;
    
    repo_metadata.branches.insert(branch_metadata.name.clone(), revision_id.clone());
    save_repo_metadata(path, &repo_metadata)?;
    
    Ok(revision_id)
}

pub fn log(path: &str) -> Result<Vec<String>, io::Error> {
    let repo_metadata = load_repo_metadata(path)?;
    let mut logs_map = HashMap::new();
    
    for (branch, _) in repo_metadata.branches.iter() {
        let branch_metadata = load_branch_metadata(path, branch)?;
        
        for revision_id in branch_metadata.commits.iter() {
            let revision_metadata = load_revision_metadata(path, branch, revision_id)?;
            let date_time: DateTime::<chrono::Local> = revision_metadata.timestamp.into();
            
            let content = format!(
                "commit {} (HEAD -> {})\nDate: {}\n\n\t{}\n\n",
                revision_metadata.id,
                branch,
                format!("{}", date_time.format("%Y-%m-%d %H:%M:%S")),
                revision_metadata.message,
            );
            
            logs_map.insert(revision_metadata.timestamp, content);
        }
    }
    
    let mut sorted_logs_map: Vec<_> = logs_map.into_iter().collect();
    sorted_logs_map.sort_by(|a, b| b.0.cmp(&a.0));
    let sorted_logs = sorted_logs_map.into_iter().map(|(_, content)| content).collect();
    Ok(sorted_logs)
}

pub fn cat(path: &str, revision_id: &str, file_name: &str) -> Result<String, io::Error> {
    let repo_metadata = load_repo_metadata(path)?;

    for (branch, _) in repo_metadata.branches.iter() {
        let branch_metadata = load_branch_metadata(path, branch)?;
        
        if branch_metadata.commits.contains(&revision_id.to_string()) {
            let revision_metadata = load_revision_metadata(path, branch, revision_id)?;
            if revision_metadata.files.contains_key(file_name) {
                let file_path = format!("{}/.dvcs/origin/{}/commits/{}/{}", path, branch, revision_id, file_name);
                let file_content = read_file(&file_path)?;

                return Ok(file_content);
            }
        } else { 
            return Err(io::Error::new(io::ErrorKind::NotFound, "Revision not found in the repository."));
        }
    }
    
    Err(io::Error::new(io::ErrorKind::NotFound, "Revision not found in the repository."))
}

pub fn checkout(path: &str, revision_id: &str) -> Result<(), io::Error> {
    let mut repo_metadata = load_repo_metadata(path)?;

    for (branch, _) in repo_metadata.branches.iter() {
        let mut branch_metadata = load_branch_metadata(path, branch)?;
        
        if branch_metadata.commits.contains(&revision_id.to_string()) {
            let revision_metadata = load_revision_metadata(path, branch, revision_id)?;
            let commit_path = format!("{}/.dvcs/origin/{}/commits/{}", path, branch, revision_id);
            let working_path = format!("{}/.dvcs/origin/{}/working", path, branch);
            
            delete_directory(&working_path, true)?;
            create_directory(&working_path)?;
            
            branch_metadata.working.clear();
            
            for (file, _) in revision_metadata.files.iter() {
                let src_path = format!("{}/{}", commit_path, file);
                let dest_path = format!("{}/{}", working_path, file);

                let content = read_file(&src_path)?;
                write_file(&dest_path, &content)?;
                branch_metadata.working.push(file.to_string());
            }
            
            repo_metadata.head = branch.to_string();
            branch_metadata.head_commit = Some(revision_id.to_string());
            
            save_branch_metadata(path, branch, &branch_metadata)?;
            save_repo_metadata(path, &repo_metadata)?;

            return Ok(());
        }
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "Revision not found in the repository.",
    ))
}

