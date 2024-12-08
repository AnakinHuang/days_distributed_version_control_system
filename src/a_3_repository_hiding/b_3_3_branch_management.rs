// days_dvcs/src/a_3_repository_hiding/b_3_3_branch_management.rs
//

use super::b_3_1_repository_management::{is_repository, load_repo_metadata, save_repo_metadata};
use super::b_3_2_revision_management::{init_revision_metadata, load_revision_metadata};

use crate::a_1_file_system_hiding::{
    b_1_1_file_interaction::{
        check_file, delete_file, get_absolute_path, get_filename, get_parent, get_relative_path,
        read_file, read_struct, write_file, write_struct,
    },
    b_1_2_directory_interaction::{
        check_directory, create_directory, delete_directory, list_directory,
    },
    REMOTE,
};

use chrono::DateTime;
use serde::{Deserialize, Serialize};
use std::collections::{HashSet, VecDeque};
use std::fmt::Debug;
use std::io;
use std::time::SystemTime;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BranchMetadata {
    pub name: String,                // Branch name
    pub head_commit: Option<String>, // Latest commit on the branch
    pub commits: Vec<String>,        // Base commit (for merges or new branches)
    pub staging: Vec<String>,        // Files staged for commit on this branch
}

pub fn is_branch(path: &str) -> Result<String, io::Error> {
    let mut branch_path = get_absolute_path("", path)?;

    loop {
        if check_directory(&branch_path) {
            let metadata_path = format!("{}/.metadata/metadata.json", branch_path);

            if check_file(&metadata_path) {
                let commits_path = format!("{}/commits", branch_path);

                if check_directory(&commits_path) {
                    let staging_path = format!("{}/staging", branch_path);

                    if check_directory(&staging_path) {
                        return Ok(branch_path);
                    }
                }
            }
        }

        let parent = get_parent(&branch_path);

        if !parent.is_empty() {
            branch_path = parent;
        } else {
            break;
        }
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        format!("No branch found starting from '{}'", get_filename(&path)),
    ))
}

pub fn init_branch(test_path: &str, branch: &str, init_repo: bool) -> Result<(), io::Error> {
    let mut path = test_path.to_string();

    if !init_repo {
        path = is_repository(test_path)?;

        if load_branch_metadata(&path, branch).is_ok() {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                format!("Branch '{}' already exists.", branch),
            ));
        }
    }

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
    save_branch_metadata(&path, branch, &init_metadata)?;

    if !init_repo {
        let mut repo_metadata = load_repo_metadata(&path)?;
        repo_metadata.head = branch.to_string();
        repo_metadata
            .branches
            .insert(branch.to_string(), String::new());
        save_repo_metadata(&path, &repo_metadata)?;
    }

    write_file(
        &format!("{}/.dvcs/HEAD", path),
        &format!("commit: {}\nref: origin/{}", "N/A", branch),
    )?;
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

pub fn add(test_path: &str, files: Vec<String>) -> Result<(), io::Error> {
    if files.len() > 1 && files.contains(&".".to_string()) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Cannot stage all files ('.') and specific files simultaneously. Use either '.' or specific file paths",
        ));
    }

    let path = &is_repository(test_path)?;
    let repo_metadata = load_repo_metadata(path)?;
    let branch = &repo_metadata.head;
    let mut branch_metadata = load_branch_metadata(path, branch)?;
    let mut files_to_stage = Vec::new();

    for file in files.iter() {
        let file_path = get_absolute_path("", file)?;

        if !file_path.starts_with(path) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Cannot add file outside repository: '{}'", file),
            ));
        }

        if check_file(&file_path) {
            files_to_stage.push(file_path.clone());
        } else if check_directory(&file_path) {
            files_to_stage.extend(list_directory(&file_path, true, true)?.into_iter().filter(
                |f| {
                    !f.strip_prefix(path).unwrap_or_default().contains(".dvcs")
                        && !f.strip_prefix(path).unwrap_or_default().contains(REMOTE)
                        && !f.strip_prefix(path).unwrap_or_default().contains(".git")
                        && !f
                            .strip_prefix(path)
                            .unwrap_or_default()
                            .contains(".DS_Store")
                },
            ));
        } else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("File or directory '{}' does not exist", file),
            ));
        }
    }

    if files_to_stage.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "No files found to stage in {}",
                files
                    .iter()
                    .map(|f| format!("'{}'", f))
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
        ));
    }

    for file in files_to_stage {
        let relative_path = get_relative_path(&file, path, true);
        let staging_dir = format!(
            "{}/.dvcs/origin/{}/staging/{}",
            path,
            branch,
            get_parent(&relative_path)
        );

        let staging_path = format!("{}/{}", staging_dir, get_filename(&file));
        let content = read_file(&file)?;

        if !branch_metadata.staging.contains(&relative_path) {
            create_directory(&staging_dir)?;
            write_file(&staging_path, &content)?;
            branch_metadata.staging.push(relative_path);
        } else if read_file(&file)? != content {
            write_file(&staging_path, &content)?; // Overwrite only if content differs
        } else {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                format!("File '{}' is already staged for commit", relative_path),
            ));
        }
    }

    save_branch_metadata(path, branch, &branch_metadata)?;
    Ok(())
}

pub fn remove(test_path: &str, files: Vec<String>) -> Result<(), io::Error> {
    let path = &is_repository(test_path)?;

    let repo_metadata = load_repo_metadata(path)?;
    let branch = &repo_metadata.head;
    let mut branch_metadata = load_branch_metadata(path, branch)?;

    if files.contains(&".".to_string()) {
        if files.len() > 1 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Cannot remove all files ('.') and specific files simultaneously. Use either '.' or specific file paths",
            ));
        }

        let stage_path = format!("{}/.dvcs/origin/{}/staging", path, branch);
        delete_directory(&stage_path, true)?;
        create_directory(&stage_path)?;
        branch_metadata.staging.clear();
    } else {
        for file in files.iter() {
            let staging_path = format!("{}/.dvcs/origin/{}/staging", path, branch);
            let file_path = get_absolute_path(file, &staging_path)?;
            let relative_path = get_relative_path(&file_path, &staging_path, true);

            if !branch_metadata.staging.contains(&relative_path) {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("File '{}' is not staged for commit", file),
                ));
            };

            if !check_file(&file_path) && !check_directory(&file_path) {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!(
                        "File or directory '{}' does not exist in the staging area",
                        file
                    ),
                ));
            }
        }

        for file in files.iter() {
            let staging_path = format!("{}/.dvcs/origin/{}/staging", path, branch,);
            let file_path = get_absolute_path(file, &staging_path)?;
            let relative_path = get_relative_path(&file_path, &staging_path, true);

            if check_file(&file_path) {
                delete_file(&file_path)?;
            } else if check_directory(&file_path) {
                delete_directory(&file_path, true)?;
            }
            branch_metadata.staging.retain(|f| f != &relative_path);
        }
    }

    save_branch_metadata(path, branch, &branch_metadata)?;
    Ok(())
}

pub fn heads(path: &str) -> Result<String, io::Error> {
    let path = &is_repository(path)?;

    let repo_metadata = load_repo_metadata(path)?;
    let mut heads = Vec::new();

    for (branch, _) in repo_metadata.branches.iter() {
        let branch_metadata = load_branch_metadata(path, branch)?;
        let mut date_time: DateTime<chrono::Local> = SystemTime::now().into();
        let mut message = "No commits yet...".to_string();
        let mut header = format!("\x1b[31morigin/{}\x1b[0m", branch);

        if branch == &repo_metadata.head {
            header = format!("\x1b[1;36mHEAD\x1b[0m \x1b[1;36m->\x1b[0m \x1b[32m{}\x1b[0m, {}, \x1b[31morigin/HEAD\x1b[0m", branch, header);
        }

        if let Some(revision_id) = &branch_metadata.head_commit {
            let revision_metadata = load_revision_metadata(path, branch, revision_id)?;
            message = revision_metadata.message;
            date_time = revision_metadata.timestamp.into();
            header = format!(
                "\x1b[33mcommit {} ({})\x1b[0m\n",
                revision_metadata.id, header
            );
        } else {
            header = format!("\x1b[33mcommit N/A ({})\x1b[0m\n", header);
        }

        let content = format!(
            "\n\x1b[0m{}Date: {}\n\n\t{}\x1b[0m\n\n",
            header,
            format!("{}", date_time.format("%Y-%m-%d %H:%M:%S")),
            message,
        );

        heads.push((date_time, content));
    }

    heads.sort_by_key(|(timestamp, _)| *timestamp);
    Ok(heads
        .into_iter()
        .rev()
        .map(|(_, content)| content)
        .collect())
}

pub fn get_common_ancestor_and_count(
    path_into: &str,
    branch_into: &str,
    revision_id_into: &str,
    path_from: &str,
    branch_from: &str,
    revision_id_from: &str,
) -> Result<(isize, Option<String>), io::Error> {
    load_revision_metadata(path_into, branch_into, revision_id_into).map_err(|e| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "'{}' commit '{}' not found: {}",
                branch_into, revision_id_into, e
            ),
        )
    })?;

    load_revision_metadata(path_from, branch_from, revision_id_from).map_err(|e| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "'{}' commit '{}' not found: {}",
                branch_from, revision_id_from, e
            ),
        )
    })?;

    if revision_id_into == revision_id_from {
        return Ok((0, Some(revision_id_into.to_string())));
    }

    let mut queue_into: VecDeque<(String, isize)> = VecDeque::new(); // Track distance
    let mut queue_from: VecDeque<(String, isize)> = VecDeque::new(); // Track distance

    let mut visited_into: HashSet<String> = HashSet::new();
    let mut visited_from: HashSet<String> = HashSet::new();

    queue_into.push_back((revision_id_into.to_string(), 0));
    queue_from.push_back((revision_id_from.to_string(), 0));

    while !queue_into.is_empty() || !queue_from.is_empty() {
        if let Some((current, distance)) = queue_into.pop_front() {
            if visited_from.contains(&current) {
                return Ok((distance, Some(current)));
            }

            if visited_into.insert(current.clone()) {
                if let Ok(metadata) = load_revision_metadata(path_into, branch_into, &current) {
                    for parent in metadata.parents {
                        queue_into.push_back((parent, distance + 1)); // Increment distance
                    }
                }
            }
        }

        if let Some((current, distance)) = queue_from.pop_front() {
            if visited_into.contains(&current) {
                return Ok((0 - distance, Some(current)));
            }
            if visited_from.insert(current.clone()) {
                if let Ok(metadata) = load_revision_metadata(path_from, branch_from, &current) {
                    for parent in metadata.parents {
                        queue_from.push_back((parent, distance + 1));
                    }
                }
            }
        }
    }

    Ok((0, None))
}

pub fn status(path: &str) -> Result<String, io::Error> {
    let repo_root = is_repository(path)?; // Get the root of the repository
    let current_path = get_absolute_path("", ".")?; // Get current working directory
    let repo_metadata = load_repo_metadata(&repo_root)?;
    let branch = &repo_metadata.head;
    let local_branch_metadata = load_branch_metadata(&repo_root, branch)?;
    let remote_repo_root = is_repository(&format!("{}/{}", repo_root, REMOTE))?;
    let remote_branch_metadata = load_branch_metadata(&remote_repo_root, branch)?;
    let mut status_report = String::new();
    status_report.push_str(&format!("On branch {}\n", branch));

    // Ahead/Behind Status
    if let Some(local_commit) = &local_branch_metadata.head_commit {
        let (count, _) = if let Some(upstream_commit) = &remote_branch_metadata.head_commit {
            get_common_ancestor_and_count(
                &repo_root,
                branch,
                local_commit,
                &remote_repo_root,
                branch,
                upstream_commit,
            )?
        } else {
            get_common_ancestor_and_count(
                &repo_root,
                branch,
                local_commit,
                &repo_root,
                branch,
                local_branch_metadata
                    .commits
                    .first()
                    .expect(&format!("No commits found in branch: {}", branch)),
            )?
        };

        if count > 0 {
            status_report.push_str(&format!(
                "Your branch is ahead of 'origin/{}' by {} commit{}.",
                branch,
                count,
                if count > 1 { "s" } else { "" }
            ));
        } else if count < 0 {
            status_report.push_str(&format!(
                "Your branch is behind 'origin/{}' by {} commit{}.",
                branch,
                count.abs(),
                if count.abs() > 1 { "s" } else { "" }
            ));
        } else {
            status_report.push_str(&format!(
                "Your branch is up to date with 'origin/{}'.",
                branch
            ));
        }
    } else {
        status_report.push_str("\nNo commits yet..."); // Please finish the status_report the same as git status
    }

    // Retrieve Latest Revision Metadata
    let latest_revision = if let Some(head_commit) = &local_branch_metadata.head_commit {
        load_revision_metadata(&repo_root, branch, head_commit)?
    } else {
        init_revision_metadata()
    };

    // Changes to Be Committed
    if !local_branch_metadata.staging.is_empty() {
        status_report.push_str(
            "\n\nChanges to be committed:\n  (use \"cargo run remove <pathspec>...\" to unstage)\n",
        );
        for file in &local_branch_metadata.staging {
            let staging_path = format!("{}/.dvcs/origin/{}/staging/{}", path, branch, file);
            let relative_path = get_relative_path(&staging_path, &current_path, false);

            if check_file(&staging_path) {
                if latest_revision.files.contains_key(file) {
                    // File exists in the latest revision
                    let revision_hash = &latest_revision.files[file];
                    let staged_content = read_file(&staging_path)?;
                    let staged_hash = format!(
                        "{:x}",
                        Uuid::new_v5(&Uuid::NAMESPACE_OID, staged_content.as_bytes())
                    );

                    if &staged_hash != revision_hash {
                        status_report
                            .push_str(&format!("\t\x1b[32mmodified:   {}\x1b[0m\n", relative_path));
                    }
                } else {
                    // File is new
                    status_report
                        .push_str(&format!("\t\x1b[32mnew file:   {}\x1b[0m\n", relative_path));
                }
            } else {
                status_report.push_str(&format!("\t\x1b[32mdeleted:   {}\x1b[0m\n", relative_path));
            }
        }
    }

    // Changes Not Staged for Commit
    let mut not_staged = Vec::new();
    for (file, hash) in &latest_revision.files {
        let full_path = format!("{}/{}", repo_root, file);
        let relative_path = get_relative_path(&full_path, &current_path, false);

        if !check_file(&full_path) {
            not_staged.push((relative_path.clone(), "deleted".to_string()));
        } else {
            let content = read_file(&full_path)?;
            let current_hash = format!(
                "{:x}",
                Uuid::new_v5(&Uuid::NAMESPACE_OID, content.as_bytes())
            );
            if current_hash != *hash {
                not_staged.push((relative_path.clone(), "modified".to_string()));
            }
        }
    }

    if local_branch_metadata.staging.is_empty() && !not_staged.is_empty() {
        status_report.push_str("\n");
    }

    if !not_staged.is_empty() {
        status_report.push_str("\nChanges not staged for commit:\n  (use \"cargo run add <pathspec>...\" to update what will be committed)\n");
        for (file, status) in &not_staged {
            status_report.push_str(&format!("\t\x1b[31m{}:   {}\x1b[0m\n", status, file));
        }
    }

    // Untracked Files
    let mut untracked_files = list_directory(&repo_root, true, true)?;
    untracked_files.retain(|file| {
        let relative_path = get_relative_path(file, &current_path, false);
        !local_branch_metadata.staging.contains(&relative_path)
            && !latest_revision.files.contains_key(&relative_path)
            && !file.contains(".dvcs")
            && !file.contains(REMOTE)
            && !file.contains(".git")
            && !file.contains(".DS_Store")
    });

    if local_branch_metadata.staging.is_empty()
        && not_staged.is_empty()
        && !untracked_files.is_empty()
    {
        status_report.push_str("\n");
    }

    if !untracked_files.is_empty() {
        status_report.push_str("\nUntracked files:\n  (use \"cargo run add <pathspec>...\" to include in what will be committed)\n");
        for file in untracked_files {
            let relative_path = get_relative_path(&file, &current_path, true);
            status_report.push_str(&format!("\t\x1b[31m{}\x1b[0m\n", relative_path));
        }
    }

    Ok(status_report)
}
