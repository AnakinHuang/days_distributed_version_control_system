// days_dvcs/src/a_1_file_system_hiding/b_1_2_directory_interaction.rs
//
//! B.1.2. Directory Interaction
//! This component is responsible for interacting with directories, 
//! and it provides functions to create, delete, and list directories.
//! 
//! Parent Module: A.1 File System Hiding
//! 
//! ## Usage:
//! The `create_directory` function creates a directory.
//! 
//! The `delete_directory` function deletes a directory.
//! 
//! The `list_directory` function lists the contents of a directory.
//! 
//! ## Dependencies:
//! - none
//!
//! Author: Anakin (Yuesong Huang), Yifan (Alvin) Jiang
//! Date: 11/14/2024

use std::fs::{self, read_dir};
use std::io;
use std::path::Path;
use crate::a_1_file_system_hiding::b_1_1_file_interaction::copy_file;

pub fn create_directory(path: &str) -> Result<(), io::Error> {
    fs::create_dir_all(path)?;
    Ok(())
}

pub fn delete_directory(path: &str, recursive: bool) -> Result<(), io::Error> {
    if Path::new(path).is_dir() && Path::new(path).exists() {
        if recursive {
            fs::remove_dir_all(path)?;
        } else {
            fs::remove_dir(path)?;
        }
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "Directory not found."))
    }
}

pub fn list_directory(path: &str) -> Result<Vec<String>, io::Error> {
    let mut entries = Vec::new();
    for entry in read_dir(path)? {
        let entry = entry?;
        let file_name = entry.file_name();
        entries.push(file_name.to_string_lossy().into_owned());
    }
    Ok(entries)
}

pub fn copy_directory(src: &str, dest: &str) -> Result<(), io::Error> {
    let src_path = Path::new(src);
    if !src_path.exists() || !src_path.is_dir() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Source is not a valid directory."));
    }

    create_directory(dest)?;
    for entry in list_directory(src)? {
        let src_path = format!("{}/{}", src, entry);
        let dest_path = format!("{}/{}", dest, entry);

        if Path::new(&src_path).is_dir() {
            copy_directory(&src_path, &dest_path)?;
        } else {
            copy_file(&src_path, &dest_path)?;
        }
    }

    Ok(())
}
