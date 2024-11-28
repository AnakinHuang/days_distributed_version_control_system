// days_dvcs/src/a_3_repository_hiding/b_3_1_repository_management.rs
//

use crate::a_1_file_system_hiding::b_1_1_file_interaction::{
    check_file, read_struct, write_file, write_struct,
};
use crate::a_1_file_system_hiding::b_1_2_directory_interaction::{check_directory, copy_directory, create_directory};
use crate::a_3_repository_hiding::b_3_3_branch_management::{init_branch, is_branch};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepositoryMetadata {
    pub head: String,                      // Current branch name
    pub branches: HashMap<String, String>, // Branch name -> commit ID
}

pub fn is_repository(path: &str) -> Result<(), io::Error> {
    let repo_path = format!("{}/.dvcs", path);

    if check_directory(&repo_path) {
        let head_path = &format!("{}/HEAD", repo_path);

        if check_file(head_path) {
            let metadata_path = &format!("{}/.metadata/metadata.json", repo_path);

            if check_file(metadata_path) {
                is_branch(path, "main")
            } else {
                Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("Repository is missing {}", metadata_path),
                ))
            }
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Repository is missing {}", head_path),
            ))
        }
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Repository is missing {}", repo_path),
        ))
    }
}

pub fn init_repository(path: &str) -> Result<(), io::Error> {
    if is_repository(path).is_ok() {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            format!("The directory {} is already a repository.", path),
        ));
    }

    let repo_path = format!("{}/.dvcs", path);
    init_branch(path, "main")?;
    write_file(&format!("{}/HEAD", repo_path), &"ref: .dvcs/origin/main")?;
    write_file(&format!("{}/HEAD", repo_path), 
               &format!(
                   "commit: {}\nref: {}/.dvcs/origin/main", 
                   "N/A", 
                   path, 
               ))?;
    create_directory(&format!("{}/.metadata", repo_path))?;
    let init_metadata = RepositoryMetadata {
        head: "main".to_string(),
        branches: HashMap::from([("main".to_string(), String::new())]),
    };
    save_repo_metadata(path, &init_metadata)?;
    Ok(())
}

pub fn clone_repository(src: &str, dest: &str) -> Result<(), io::Error> {
    if is_repository(src).is_err() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("The directory {} is not a repository.", src),
        ));
    }

    if is_repository(dest).is_ok() {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            format!("The directory {} already exists and it is not empty.", dest),
        ));
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
