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

use std::fs::{metadata, Metadata};
use std::io;
use std::os::unix::fs::PermissionsExt;
use std::time::SystemTime;

#[derive(Debug)]
pub struct FileMetadata {
    pub size: u64,
    pub last_modified: SystemTime,
    pub is_directory: bool,
    pub mode: u32,
}

pub fn get_file_metadata(path: &str) -> Result<FileMetadata, io::Error> {
    let metadata = get_metadata(path)?;
    let file_metadata = FileMetadata {
        size: metadata.len(),
        last_modified: match get_last_modified(&metadata) {
            Ok(time) => time,
            Err(e) => {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("Failed to get metadata '{}': {}", path, e),
                ))
            }
        },
        is_directory: metadata.is_dir(),
        mode: get_mode(&metadata)?,
    };
    Ok(file_metadata)
}

fn get_metadata(path: &str) -> Result<Metadata, io::Error> {
    match metadata(path) {
        Ok(metadata) => Ok(metadata),
        Err(e) => Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Failed to get metadata '{}': {}", path, e),
        )),
    }
}

fn get_last_modified(metadata: &Metadata) -> Result<SystemTime, io::Error> {
    match metadata.modified() {
        Ok(time) => Ok(time),
        Err(e) => Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Failed to get last modified time: {}", e),
        )),
    }
}

fn get_mode(metadata: &Metadata) -> Result<u32, io::Error> {
    Ok(metadata.permissions().mode() & 0o777_777)
}
