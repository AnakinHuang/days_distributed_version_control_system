// days_dvcs/src/a_1_file_system_hiding/b_1_3_metadata_management.rs
//
//! B.1.3 Metadata Management
//! This component is responsible for managing file metadata, 
//! and it provides functions to get file metadata, file size, and last modified time.
//!
//! Parent Module: A.1 File System Hiding
//! 
//! ## Usage:
//! The `get_file_metadata` function gets the metadata of a file.
//! 
//! The `get_file_size` function gets the size of a file.
//! 
//! The `get_last_modified` function gets the last modified time of a file.
//! 
//! ## Dependencies:
//! - none
//!
//! Author: Anakin (Yuesong Huang), Yifan (Alvin) Jiang
//! Date: 11/14/2024

use std::fs::{metadata};
use std::io;
use std::time::SystemTime;

#[derive(Debug)]
pub struct FileMetadata {
    pub size: u64,
    pub last_modified: SystemTime,
    pub is_directory: bool,
}

pub fn get_file_metadata(path: &str) -> Result<FileMetadata, io::Error> {
    let metadata = metadata(path)?;
    let file_metadata = FileMetadata {
        size: metadata.len(),
        last_modified: metadata.modified()?,
        is_directory: metadata.is_dir(),
    };
    Ok(file_metadata)
}

pub fn get_file_size(path: &str) -> Result<u64, io::Error> {
    let metadata = metadata(path)?;
    Ok(metadata.len())
}

pub fn get_last_modified(path: &str) -> Result<SystemTime, io::Error> {
    let metadata = metadata(path)?;
    Ok(metadata.modified()?)
}
