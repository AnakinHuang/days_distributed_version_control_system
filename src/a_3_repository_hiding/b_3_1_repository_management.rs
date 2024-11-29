// days_dvcs/src/a_3_repository_hiding/b_3_1_repository_management.rs
//

use crate::a_1_file_system_hiding::b_1_1_file_interaction::{
    check_file, get_absolute_path, get_parent, read_struct, write_struct,
};
use crate::a_1_file_system_hiding::b_1_2_directory_interaction::{
    check_directory, copy_directory, create_directory, is_empty_directory,
};
use crate::a_3_repository_hiding::b_3_3_branch_management::{init_branch, is_branch};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepositoryMetadata {
    pub head: String,                      // Current branch name
    pub branches: HashMap<String, String>, // Branch name -> commit ID
}

pub fn is_repository(path: &str) -> Result<String, io::Error> {
    let mut absolute_path = get_absolute_path("", path)?;
    let mut error;

    loop {
        let repo_path = format!("{}/.dvcs", absolute_path);

        if check_directory(&repo_path) {
            let head_path = &format!("{}/HEAD", repo_path);

            if check_file(head_path) {
                let metadata_path = &format!("{}/.metadata/metadata.json", repo_path);

                if check_file(metadata_path) {
                    if is_branch(&format!("{}/origin/main", repo_path)).is_ok() {
                        return Ok(absolute_path);
                    } else {
                        error = io::Error::new(
                            io::ErrorKind::NotFound,
                            format!("No main branch found in repository {}", repo_path),
                        );
                    }
                } else {
                    error = io::Error::new(
                        io::ErrorKind::NotFound,
                        format!("No metadata file found in repository {}", repo_path),
                    );
                }
            } else {
                error = io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("No HEAD file found in repository {}", repo_path),
                );
            }
        } else {
            error = io::Error::new(
                io::ErrorKind::NotFound,
                format!("No repository found starting from {}", path),
            );
        }

        let parent = get_parent(&absolute_path);

        if !parent.is_empty() {
            absolute_path = parent;
        } else {
            break;
        }
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        format!("No repository found starting from {}\n{}", path, error),
    ))
}

pub fn init_repository(path: &str) -> Result<(), io::Error> {
    match is_repository(path) {
        Ok(root_path) => {
            let current_path = get_absolute_path("", path)?;

            if root_path == current_path {
                return Err(io::Error::new(
                    io::ErrorKind::AlreadyExists,
                    format!("The directory {} is already a repository.", path),
                ));
            }
        }
        Err(_) => {}
    }

    let repo_path = format!("{}/.dvcs", path);
    create_directory(&format!("{}/.metadata", repo_path))?;
    init_branch(&path, "main", true)?;
    let init_metadata = RepositoryMetadata {
        head: "main".to_string(),
        branches: HashMap::from([("main".to_string(), String::new())]),
    };
    save_repo_metadata(&path, &init_metadata)?;
    Ok(())
}

pub fn clone_repository(src: &str, dest: &str) -> Result<(), io::Error> {
    match is_repository(src) {
        Ok(root_path) => {
            let current_path = get_absolute_path("", src)?;

            if root_path != current_path {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("The directory {} is not a repository.", src),
                ));
            }
        }
        Err(_) => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("The directory {} is not a repository.", src),
            ));
        }
    }

    if check_directory(dest) {
        if is_empty_directory(dest).is_err() {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                format!("The directory {} already exists and is not empty.", dest),
            ));
        }
    }

    copy_directory(src, dest)?;
    Ok(())
}

pub fn save_repo_metadata(path: &str, metadata: &RepositoryMetadata) -> Result<(), io::Error> {
    let metadata_path = format!("{}/.dvcs/.metadata/metadata.json", path);
    write_struct(&metadata_path, metadata)?;
    Ok(())
}

pub fn load_repo_metadata(path: &str) -> Result<RepositoryMetadata, io::Error> {
    let metadata_path = format!("{}/.dvcs/.metadata/metadata.json", path);
    let metadata: RepositoryMetadata = read_struct(&metadata_path)?;
    Ok(metadata)
}
