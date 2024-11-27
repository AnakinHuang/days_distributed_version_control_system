// days_dvcs/src/a_3_repository_hiding/b_3_1_repository_management.rs
//

use crate::a_1_file_system_hiding::b_1_1_file_interaction::{check_file, read_struct, write_file, write_struct};
use crate::a_1_file_system_hiding::b_1_2_directory_interaction::{create_directory, copy_directory};
use crate::a_3_repository_hiding::b_3_3_branch_management::init_branch;

use std::io;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepositoryMetadata {
    pub head: String,                     // Current branch name
    pub branches: HashMap<String, String>, // Branch name -> commit ID
}

pub fn init_repository(path: &str) -> Result<(), io::Error> {
    if check_file(path) {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            format!("Repository already exists at {}", path),
        ));
    }

    let repo_path = format!("{}/.dvcs", path);
    init_branch(path, "main")?;
    create_directory(&format!("{}/.metadata", repo_path))?;
    write_file(&format!("{}/HEAD", repo_path), &"ref: .dvcs/origin/main")?;

    let init_metadata = RepositoryMetadata {
        head: "main".to_string(),
        branches: HashMap::from([("main".to_string(), String::new())]),
    };

    save_repo_metadata(path, &init_metadata)?;
    Ok(())
}

pub fn clone_repository(src: &str, dest: &str) -> Result<(), io::Error> {
    let metadata_path = format!("{}/.dvcs/.metadata/metadata.json", src);
    if !check_file(&metadata_path) {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Source repository metadata not found at {}", metadata_path),
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
