// days_dvcs/src/a_3_repository_hiding/b_3_3_branch_management.rs
//

use std::collections::HashMap;
use crate::a_1_file_system_hiding::b_1_1_file_interaction::{
    get_filename, get_parent, get_relative_path, check_file, delete_file, read_file, read_struct, write_struct,
};
use crate::a_1_file_system_hiding::b_1_2_directory_interaction::{check_directory, create_directory, delete_directory, list_directory};
use crate::a_3_repository_hiding::b_3_1_repository_management::{
    is_repository, load_repo_metadata,
};
#[allow(unused_imports)]
use crate::a_3_repository_hiding::b_3_2_revision_management::{init_revision_metadata, load_revision_metadata};

use chrono::DateTime;
use serde::{Deserialize, Serialize};
use std::io;
use std::time::SystemTime;
use crate::a_2_behavioral_hiding::b_2_2_command_handler::RevisionMetadata;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BranchMetadata {
    pub name: String,                // Branch name
    pub head_commit: Option<String>, // Latest commit on the branch
    pub commits: Vec<String>,        // Base commit (for merges or new branches)
    pub staging: Vec<String>,        // Files staged for commit on this branch
}

pub fn is_branch(path: &str, branch: &str) -> Result<(), io::Error> {
    let branch_path = format!("{}/.dvcs/origin/{}", path, branch);
    
    if check_directory(&branch_path) {
        let metadata_path = format!("{}/.metadata/metadata.json", branch_path);

        if check_file(&metadata_path) {
            let commits_path = format!("{}/commits", branch_path);

            if check_directory(&commits_path) {
                let staging_path = format!("{}/staging", branch_path);

                if check_directory(&staging_path) {
                    Ok(())
                } else {
                    Err(io::Error::new(
                        io::ErrorKind::NotFound,
                        format!("Repository main Branch is missing {}", staging_path),
                    ))
                }
            } else {
                Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("Repository main Branch is missing {}", commits_path),
                ))
            }
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Repository main Branch is missing {}", metadata_path),
            ))
        }
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Repository main Branch is missing {}", branch_path),
        ))
    }
}

pub fn init_branch(path: &str, branch: &str) -> Result<(), io::Error> {
    let branch_path = format!("{}/.dvcs/origin/{}", path, branch);
    create_directory(&format!("{}/commits", branch_path))?;
    create_directory(&format!("{}/staging", branch_path))?;
    create_directory(&format!("{}/.metadata", branch_path))?;
    let init_metadata = BranchMetadata {
        name: branch.to_string(),
        head_commit: None,
        commits: Vec::new(),
        staging: Vec::new(),
    };
    save_branch_metadata(path, branch, &init_metadata)?;
    Ok(())
}

pub fn load_branch_metadata(path: &str, branch: &str) -> Result<BranchMetadata, io::Error> {
    let metadata_path = format!("{}/.dvcs/origin/{}/.metadata/metadata.json", path, branch);
    let metadata: BranchMetadata = read_struct(&metadata_path)?;
    Ok(metadata)
}

pub fn save_branch_metadata(
    path: &str,
    branch: &str,
    metadata: &BranchMetadata,
) -> Result<(), io::Error> {
    let metadata_path = format!("{}/.dvcs/origin/{}/.metadata/metadata.json", path, branch);
    write_struct(&metadata_path, metadata)?;
    Ok(())
}

pub fn add(path: &str, files: Vec<String>) -> Result<(), io::Error> {
    is_repository(path)?;

    if files.len() > 1 && files.contains(&".".to_string()) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Cannot stage all files ('.') and specific files simultaneously. Use either '.' or specific file paths.",
        ));
    }

    let repo_metadata = load_repo_metadata(path)?;
    let branch = &repo_metadata.head;
    let mut branch_metadata = load_branch_metadata(path, branch)?;
    let mut files_to_stage = Vec::new();

    for file in files.iter() {
        if check_file(&file) {
            files_to_stage.push(file.clone());
        } else if check_directory(&file) {
            files_to_stage.extend(list_directory(file, true, true)?);
        } else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("File or directory '{}' does not exist.", file),
            ));
        }
    }
    
    let files_to_stage = files_to_stage.into_iter()
        .filter(|f| !f.contains(".dvcs")) // Ignore .dvcs and its contents
        .collect::<Vec<_>>();

    if files_to_stage.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "No valid files to stage."
        ));
    }

    for file in files_to_stage {
        let relative_path = get_relative_path(&file, path);
        let staging_dir = format!(
            "{}/.dvcs/origin/{}/staging/{}", 
            path, 
            branch, 
            get_parent(&relative_path)
        );
        let staging_path = format!(
            "{}/{}", 
            staging_dir,
            get_filename(&file)
        );
        let content = read_file(&file)?;
        
        if !branch_metadata.staging.contains(&relative_path) {
            create_directory(&staging_dir)?;
            write_struct(&staging_path, &content)?;
            branch_metadata.staging.push(relative_path);
        } else if read_file(&staging_path)? != content {
            write_struct(&staging_path, &content)?; // Overwrite only if content differs
        }
    }

    save_branch_metadata(path, branch, &branch_metadata)?;
    Ok(())
}

pub fn remove(path: &str, files: Vec<String>) -> Result<(), io::Error> {
    is_repository(path)?;

    let repo_metadata = load_repo_metadata(path)?;
    let branch = &repo_metadata.head;
    let mut branch_metadata = load_branch_metadata(path, branch)?;
    
    if files.contains(&".".to_string()) {
        if files.len() > 1 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Cannot remove all files ('.') and specific files simultaneously. Use either '.' or specific file paths.",
            ));
        }

        let stage_path = format!("{}/.dvcs/origin/{}/staging", path, branch);
        delete_directory(&stage_path, true)?;
        create_directory(&stage_path)?;
        branch_metadata.staging.clear();
    } else {
        for file in files.iter() {
            let relative_path = get_relative_path(file, path);
            let staging_path = format!("{}/.dvcs/origin/{}/staging/{}", path, branch, relative_path);

            if !branch_metadata.staging.contains(&relative_path) {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("File '{}' is not staged for commit.", file),
                ));
            }

            if !check_file(&staging_path) && !check_directory(&staging_path) {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("File or directory '{}' does not exist in the staging area.", relative_path),
                ));
            }
        }
        
        for file in files {
            let relative_path = get_relative_path(&file, path);
            let staging_path = format!(
                "{}/.dvcs/origin/{}/staging/{}", 
                path, 
                branch, 
                relative_path
            );
            
            if check_file(&staging_path) {
                delete_file(&staging_path)?;
            } else if check_directory(&staging_path) {
                delete_directory(&staging_path, true)?;
            }
            branch_metadata.staging.retain(|f| f != &relative_path);
        }
    }
    
    save_branch_metadata(path, branch, &branch_metadata)?;
    Ok(())
}

pub fn heads(path: &str) -> Result<String, io::Error> {
    is_repository(path)?;

    let repo_metadata = load_repo_metadata(path)?;
    let mut heads = Vec::new();

    for (branch, _) in repo_metadata.branches.iter() {
        let branch_metadata = load_branch_metadata(path, branch)?;

        if let Some(revision_id) = branch_metadata.head_commit.as_ref() {
            let revision_metadata = load_revision_metadata(path, branch, revision_id)?;
            let date_time: DateTime<chrono::Local> = revision_metadata.timestamp.into();

            let content = format!(
                "\ncommit {} ({}, origin/{})\nDate: {}\n\n\t{}\n",
                revision_metadata.id,
                branch,
                branch,
                format!("{}", date_time.format("%Y-%m-%d %H:%M:%S")),
                revision_metadata.message,
            );

            heads.push((revision_metadata.timestamp, content));
        } else {
            let date_time: DateTime<chrono::Local> = SystemTime::now().into();
            
            let content = format!(
                "commits N/A ({}, origin/{})\nDate: {}\n\n\t{}\n",
                branch,
                branch,
                format!("{}", date_time.format("%Y-%m-%d %H:%M:%S")),
                "No commits yet...",
            );

            heads.push((date_time.into(), content));
        }
    }

    heads.sort_by_key(|(timestamp, _)| *timestamp);
    Ok(heads.into_iter().map(|(_, content)| content).collect())
}

pub fn status(path: &str) -> Result<String, io::Error> {
    is_repository(path)?;

    let repo_metadata = load_repo_metadata(path)?;
    let branch = &repo_metadata.head;
    let branch_metadata = load_branch_metadata(path, branch)?;
    let mut status_report = String::new();
    status_report.push_str(&format!("On branch {}\n", branch));

    // Ahead/Behind Status
    if let Some(upstream_commit) = branch_metadata.head_commit.as_ref() {
        if let Some(local_commit) = repo_metadata.branches.get(branch) {
            let ahead_count = count_commits_ahead(path, branch, local_commit, upstream_commit)?;
            if ahead_count > 0 {
                status_report.push_str(&format!(
                    "Your branch is ahead of 'origin/{}' by {} commit{}.\n",
                    branch, ahead_count, if ahead_count > 1 { "s" } else { "" }
                ));
            } else {
                status_report.push_str("Your branch is up to date with 'origin/'.\n");
            }
        }
    } else {
        status_report.push_str("No upstream branch.\n");
    }

    // Retrieve Latest Revision Metadata
    let latest_revision = if let Some(head_commit) = branch_metadata.head_commit.as_ref() {
        load_revision_metadata(path, branch, head_commit)?
    } else {
        RevisionMetadata {
            id: String::new(),
            timestamp: SystemTime::now(),
            message: String::new(),
            files: HashMap::new(),
            parents: Vec::new(),
        }
    };

    // Changes to Be Committed
    if !branch_metadata.staging.is_empty() {
        status_report.push_str("\nChanges to be committed:\n  (use \"cargo run remove <pathspec>...\" to unstage)\n");
        for file in &branch_metadata.staging {
            let staging_path = format!("{}/.dvcs/origin/{}/staging/{}", path, branch, file);

            if check_file(&staging_path) {
                status_report.push_str(&format!("\tnew file:   {}\n", file));
            } else if check_directory(&staging_path) {
                status_report.push_str(&format!("\tnew directory:   {}\n", file));
            }
        }
    }

    // Changes Not Staged for Commit
    let mut not_staged = Vec::new();
    for (file, hash) in &latest_revision.files {
        let full_path = format!("{}/{}", path, file);
        if !check_file(&full_path) {
            not_staged.push((file.clone(), "deleted".to_string()));
        } else {
            let content = read_file(&full_path)?;
            let current_hash = format!("{:x}", md5::compute(content));
            if current_hash != *hash {
                not_staged.push((file.clone(), "modified".to_string()));
            }
        }
    }

    if !not_staged.is_empty() {
        status_report.push_str("\nChanges not staged for commit:\n  (use \"cargo run add <pathspec>...\" to update what will be committed)\n");
        for (file, status) in &not_staged {
            status_report.push_str(&format!("\t{}:   {}\n", status, file));
        }
    }

    // Untracked Files
    let mut untracked_files = list_directory(path, true, true)?;
    untracked_files.retain(|file| {
        !branch_metadata.staging.contains(file)
            && !latest_revision.files.contains_key(file)
            && !file.contains(".dvcs")
    });

    if !untracked_files.is_empty() {
        status_report.push_str("\nUntracked files:\n  (use \"cargo run add <pathspec>...\" to include in what will be committed)\n");
        for file in untracked_files {
            status_report.push_str(&format!("\t{}\n", file));
        }
    }

    Ok(status_report)
}

fn count_commits_ahead(path: &str, branch: &str, local: &str, upstream: &str) -> Result<usize, io::Error> {
    let mut ahead = 0;
    let mut current = local.to_string();

    while current != upstream {
        let current_metadata = load_revision_metadata(path, branch, &current)
            .map_err(|e| io::Error::new(e.kind(), format!("Failed to load metadata for commit '{}': {}", current, e)))?;

        if let Some(parent) = current_metadata.parents.first() {
            current = parent.clone();
            ahead += 1;
        } else {
            break;
        }
    }

    Ok(ahead)
}
