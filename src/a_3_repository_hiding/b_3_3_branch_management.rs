// days_dvcs/src/a_3_repository_hiding/b_3_3_branch_management.rs
//

use crate::a_1_file_system_hiding::b_1_1_file_interaction::write_struct;
use crate::a_1_file_system_hiding::b_1_2_directory_interaction::create_directory;
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
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
        staging: Vec::from(["program.rs".to_string()]),
        working: Vec::new(),
    };
    
    save_branch_metadata(path, branch, &init_metadata)?;
    Ok(())
}

pub fn load_branch_metadata(path: &str, branch: &str) -> Result<BranchMetadata, std::io::Error> {
    let metadata_path = format!("{}/.dvcs/origin/{}/.metadata/metadata.json", path, branch);
    let metadata: BranchMetadata = crate::a_1_file_system_hiding::b_1_1_file_interaction::read_struct(&metadata_path)?;
    Ok(metadata)
}

pub fn save_branch_metadata(path: &str, branch: &str, metadata: &BranchMetadata) -> Result<(), std::io::Error> {
    let metadata_path = format!("{}/.dvcs/origin/{}/.metadata/metadata.json", path, branch);
    println!("{:?}", metadata_path);
    write_struct(&metadata_path, metadata)?;
    Ok(())
}