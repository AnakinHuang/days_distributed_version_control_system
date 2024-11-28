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

use super::b_1_1_file_interaction::{check_file, copy_file};
use std::fs::{canonicalize, read_dir, create_dir_all, remove_dir, remove_dir_all};
use std::io;
use std::path::Path;

pub fn check_directory(path: &str) -> bool {
    Path::new(path).is_dir() || match canonicalize(path) { 
        Ok(path) => path.is_dir(),
        Err(_) => false,
    }
}

pub fn create_directory(path: &str) -> Result<(), io::Error> {
    create_dir_all(path)?;
    Ok(())
}

pub fn delete_directory(path: &str, recursive: bool) -> Result<(), io::Error> {
    if Path::new(path).is_dir() {
        if recursive {
            remove_dir_all(path)?;
        } else {
            remove_dir(path)?;
        }
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Directory not found.",
        ))
    }
}

pub fn list_directory(path: &str, full: bool, recursive: bool) -> Result<Vec<String>, io::Error> {
    let mut entries = Vec::new();
    if recursive {
        list_directory_helper(path, full, &mut entries)?;
    } else {
        for entry in read_dir(path)? {
            let entry = entry?;
            let file_name = entry.file_name();
            if full {
                entries.push(format!("{}/{}", path, file_name.to_string_lossy()));
            } else {
                entries.push(file_name.to_string_lossy().into_owned());
            }
        }
    }
    Ok(entries)
}

fn list_directory_helper(path: &str, full: bool, files: &mut Vec<String>) -> Result<(), io::Error> {
    for entry in read_dir(path)? {
        let entry = entry?;
        let full_path = format!("{}/{}", path, entry.file_name().to_string_lossy());

        if check_directory(&full_path) {
            list_directory_helper(&full_path, full, files)?;
        } else if check_file(&full_path) {
            if full {
                files.push(full_path);
            } else {
                files.push(entry.file_name().to_string_lossy().into_owned());
            }
        }
    }
    Ok(())
}

pub fn copy_directory(src: &str, dest: &str) -> Result<(), io::Error> {
    let src_path = Path::new(src);
    if !src_path.exists() || !src_path.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Source is not a valid directory",
        ));
    }

    create_directory(dest)?;
    for entry in list_directory(src, false, false)? {
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
