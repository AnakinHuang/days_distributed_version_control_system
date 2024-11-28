// days_dvcs/src/a_3_repository_hiding/b_3_2_revision_management.rs
//

use crate::a_1_file_system_hiding::b_1_1_file_interaction::{check_file, get_parent, read_file, read_struct, write_file, write_struct};
use crate::a_1_file_system_hiding::b_1_2_directory_interaction::{check_directory, create_directory, delete_directory};
use crate::a_3_repository_hiding::b_3_1_repository_management::{
    load_repo_metadata, save_repo_metadata, RepositoryMetadata,
};
use crate::a_3_repository_hiding::b_3_3_branch_management::{
    load_branch_metadata, save_branch_metadata,
};

use crate::a_2_behavioral_hiding::b_2_2_command_handler::is_repository;
use chrono::DateTime;
use md5;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use std::io;
use std::time::SystemTime;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RevisionMetadata {
    pub id: String,                     // Unique identifier for the revision
    pub files: HashMap<String, String>, // Maps file paths to their content hashes
    pub parents: Vec<String>,           // Parent revisions (supports merges)
    pub message: String,                // Commit message
    pub timestamp: SystemTime,          // Timestamp of the commit
}

pub fn is_revision(path: &str, branch: &str, revision_id: &str) -> Result<(), io::Error> {
    let revision_path = format!("{}/.dvcs/origin/{}/commits/{}", path, branch, revision_id);
    let metadata_path = format!("{}/.metadata/metadata.json", revision_path);

    if !check_file(&revision_path) {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Revision does not exist {}", revision_path),
        ));
    }

    if !check_file(&metadata_path) {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Revision is missing {}", metadata_path),
        ));
    }

    Ok(())
}

pub fn init_revision_metadata() -> RevisionMetadata {
    RevisionMetadata {
        id: String::new(),
        files: HashMap::new(),
        parents: Vec::new(),
        message: String::new(),
        timestamp: SystemTime::now(),
    }
}

pub fn load_revision_metadata(
    path: &str,
    branch: &str,
    revision_id: &str,
) -> Result<RevisionMetadata, io::Error> {
    let metadata_path = format!(
        "{}/.dvcs/origin/{}/commits/{}/.metadata/metadata.json",
        path, branch, revision_id
    );
    let metadata: RevisionMetadata = read_struct(&metadata_path)?;
    Ok(metadata)
}

pub fn save_revision_metadata(
    path: &str,
    branch: &str,
    revision_id: &str,
    metadata: &RevisionMetadata,
) -> Result<(), io::Error> {
    let metadata_path = format!(
        "{}/.dvcs/origin/{}/commits/{}/.metadata/metadata.json",
        path, branch, revision_id
    );
    write_struct(&metadata_path, metadata)?;
    Ok(())
}

pub fn commit(path: &str, message: &str) -> Result<String, io::Error> {
    is_repository(path)?;

    let mut repo_metadata = load_repo_metadata(path)?;
    let branch = &repo_metadata.head;
    let mut branch_metadata = load_branch_metadata(path, branch)?;

    if branch_metadata.staging.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "No changes to commit.",
        ));
    }

    let revision_id = Uuid::new_v4().to_string();
    let staged_path = format!("{}/.dvcs/origin/{}/staging", path, branch);
    let commit_path = format!("{}/.dvcs/origin/{}/commits/{}", path, branch, revision_id);
    
    // Create the commit directory
    create_directory(&commit_path)?;

    let mut files = HashMap::new();
    
    // Prepare files and compute their hashes
    for file in &branch_metadata.staging {
        let src_path = format!("{}/{}", staged_path, file);
        let dest_path = format!("{}/{}", commit_path, file);
        let dest_dir = get_parent(&dest_path);
        
        if !check_directory(&dest_dir) {
            create_directory(&dest_dir)?;
        }

        if check_file(&src_path) {
            let content = read_file(&src_path)?;
            write_file(&dest_path, &content)?; // Copy the file content to the commit
            let file_hash = format!("{:x}", md5::compute(content));
            files.insert(file.clone(), file_hash);
        } else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Staged file '{}' does not exist.", src_path),
            ));
        }
    }

    // Create new revision metadata
    let new_revision = RevisionMetadata {
        id: revision_id.clone(),
        files,
        parents: branch_metadata
            .head_commit
            .clone()
            .map(|p| vec![p])
            .unwrap_or_default(),
        message: message.to_string(),
        timestamp: SystemTime::now(),
    };

    // Save the revision metadata
    create_directory(&format!("{}/.metadata", commit_path))?;
    save_revision_metadata(path, &repo_metadata.head, &revision_id, &new_revision)?;

    // Clear the staging area after committing
    delete_directory(&staged_path, true)?;
    create_directory(&staged_path)?;

    // Update branch metadata
    branch_metadata.head_commit = Some(revision_id.clone());
    branch_metadata.commits.push(revision_id.clone());
    branch_metadata.staging.clear();
    save_branch_metadata(path, branch, &branch_metadata)?;

    // Update repository metadata
    repo_metadata.branches.insert(branch.clone(), revision_id.clone());
    save_repo_metadata(path, &repo_metadata)?;

    // Update HEAD
    write_file(
        &format!("{}/.dvcs/HEAD", path),
        &format!(
            "commit: {}\nref: {}/.dvcs/origin/{}",
            revision_id, path, branch
        ),
    )?;
    Ok(revision_id)
}

pub fn log(path: &str) -> Result<String, io::Error> {
    is_repository(path)?;

    let repo_metadata = load_repo_metadata(path)?;
    let head_branch = &repo_metadata.head;
    let mut logs = Vec::new();

    for (branch, _) in repo_metadata.branches.iter() {
        let branch_metadata = load_branch_metadata(path, branch)?;

        for revision_id in branch_metadata.commits.iter() {
            let revision_metadata = load_revision_metadata(path, branch, revision_id)?;
            let date_time: DateTime<chrono::Local> = revision_metadata.timestamp.into();

            let header = if branch == head_branch {
                format!("HEAD -> {}", head_branch)
            } else {
                format!("origin/{}", branch)
            };

            let content = format!(
                "commit {} ({})\nDate: {}\n\n\t{}\n",
                revision_metadata.id,
                header,
                format!("{}", date_time.format("%Y-%m-%d %H:%M:%S")),
                revision_metadata.message,
            );

            logs.push((revision_metadata.timestamp, content));
        }
    }

    logs.sort_by_key(|(timestamp, _)| *timestamp);

    if logs.is_empty() {
        Ok("No commits yet...".to_string())
    } else {
        Ok(logs.into_iter().map(|(_, content)| content).collect())
    }
}

fn get_revision_id(
    path: &str,
    revision_id: &str,
) -> Result<(RepositoryMetadata, String), io::Error> {
    let repo_metadata = load_repo_metadata(path)?;

    if revision_id.is_empty() {
        let head_branch_metadata = load_branch_metadata(path, &repo_metadata.head)?;

        if head_branch_metadata.head_commit.is_none() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "No commits in the current branch.",
            ));
        }

        return Ok((repo_metadata, head_branch_metadata.head_commit.unwrap()));
    }

    Ok((repo_metadata, revision_id.to_string()))
}

pub fn cat(path: &str, revision_id: &str, file_name: &str) -> Result<String, io::Error> {
    is_repository(path)?;

    let (repo_metadata, last_revision_id) = get_revision_id(path, revision_id)?;

    for (branch, _) in repo_metadata.branches.iter() {
        let branch_metadata = load_branch_metadata(path, branch)?;

        if branch_metadata.commits.contains(&last_revision_id) {
            let revision_metadata = load_revision_metadata(path, branch, &last_revision_id)?;
            if revision_metadata.files.contains_key(file_name) {
                let file_path = format!(
                    "{}/.dvcs/origin/{}/commits/{}/{}",
                    path, branch, last_revision_id, file_name
                );
                let file_content = read_file(&file_path)?;

                return Ok(file_content);
            }
        } else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Revision not found in the repository.",
            ));
        }
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "Revision not found in the repository.",
    ))
}

fn get_branch_or_revision_id(
    path: &str,
    branch_or_revision_id: &str,
) -> Result<(RepositoryMetadata, String), io::Error> {
    is_repository(path)?;

    let repo_metadata = load_repo_metadata(path)?;

    if repo_metadata.branches.contains_key(branch_or_revision_id) {
        let revision_id = repo_metadata
            .branches
            .get(branch_or_revision_id)
            .unwrap()
            .to_string();
        return Ok((repo_metadata, revision_id));
    }
    
    get_revision_id(path, branch_or_revision_id)
}

pub fn checkout(path: &str, branch_or_revision_id: &str) -> Result<(), io::Error> {
    is_repository(path)?;

    let (mut repo_metadata, last_revision_id) =
        get_branch_or_revision_id(path, branch_or_revision_id)?;
    
    for (branch, _) in repo_metadata.branches.iter() {
        let mut branch_metadata = load_branch_metadata(path, branch)?;

        if branch_metadata.commits.contains(&last_revision_id) || branch == branch_or_revision_id {
            let revision_metadata = load_revision_metadata(path, branch, &last_revision_id)?;
            let commit_path = format!(
                "{}/.dvcs/origin/{}/commits/{}",
                path, branch, &last_revision_id
            );
            
            if !check_directory(path) {
                create_directory(path)?;
            }

            for (file, _) in revision_metadata.files.iter() {
                let src_path = format!("{}/{}", commit_path, file);
                let dest_path = format!("{}/{}", path, file);

                let content = read_file(&src_path)?;
                write_file(&dest_path, &content)?;
            }

            repo_metadata.head = branch.to_string();
            branch_metadata.head_commit = Some(last_revision_id.to_string());

            save_branch_metadata(path, branch, &branch_metadata)?;
            save_repo_metadata(path, &repo_metadata)?;
            write_file(&format!("{}/.dvcs/HEAD", path), 
                       &format!("commit: {}\nref: {}/.dvcs/origin/{}", 
                                branch_metadata.head_commit.unwrap_or("".to_string()), 
                                path, 
                                branch))?;
            return Ok(())
        }
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "Revision not found in the repository.",
    ))
}
