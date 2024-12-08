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

use std::fs::{canonicalize, create_dir_all, read_dir, remove_dir, remove_dir_all, rename};
use std::io;
use std::path::Path;

pub fn check_directory(path: &str) -> bool {
    Path::new(path).is_dir()
        || match canonicalize(path) {
            Ok(path) => path.is_dir(),
            Err(_) => false,
        }
}

pub fn rename_directory(old_path: &str, new_path: &str) -> Result<(), io::Error> {
    if Path::new(old_path).is_dir() {
        rename(old_path, new_path)
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "Failed to rename directory '{}': Directory not found",
                old_path
            ),
        ))
    }
}

pub fn create_directory(path: &str) -> Result<(), io::Error> {
    match create_dir_all(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            format!("Failed to create directory '{}': {}", path, e),
        )),
    }
}

pub fn delete_directory(path: &str, recursive: bool) -> Result<(), io::Error> {
    if Path::new(path).is_dir() {
        if recursive {
            match remove_dir_all(path) {
                Ok(_) => Ok(()),
                Err(e) => Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("Failed to delete directory '{}': {}", path, e),
                )),
            }
        } else {
            match remove_dir(path) {
                Ok(_) => Ok(()),
                Err(e) => Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("Failed to delete directory '{}': {}", path, e),
                )),
            }
        }
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Failed to delete directory '{}': Directory not found", path),
        ))
    }
}

pub fn is_empty_directory(path: &str) -> Result<(), io::Error> {
    match read_dir(path) {
        Ok(mut dir) => {
            if dir.next().is_none() {
                Ok(())
            } else {
                Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!(
                        "Failed to delete directory '{}': Directory is not empty",
                        path
                    ),
                ))
            }
        }
        Err(e) => Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Failed to read directory '{}': {}", path, e),
        )),
    }
}

pub fn list_directory(path: &str, full: bool, recursive: bool) -> Result<Vec<String>, io::Error> {
    let mut entries = Vec::new();
    if recursive {
        list_directory_helper(path, full, &mut entries)?;
    } else {
        match read_dir(path) {
            Ok(dir) => {
                for entry in dir {
                    match entry {
                        Ok(entry) => {
                            let file_name = entry.file_name();

                            if full {
                                entries.push(format!("{}/{}", path, file_name.to_string_lossy()));
                            } else {
                                entries.push(file_name.to_string_lossy().into_owned());
                            }
                        }
                        Err(e) => {
                            return Err(io::Error::new(
                                io::ErrorKind::NotFound,
                                format!("Failed to list directory '{}': {}", path, e),
                            ))
                        }
                    }
                }
            }
            Err(e) => {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("Failed to list directory '{}': {}", path, e),
                ))
            }
        }
    }

    Ok(entries)
}

fn list_directory_helper(path: &str, full: bool, files: &mut Vec<String>) -> Result<(), io::Error> {
    match read_dir(path) {
        Ok(dir) => {
            for entry in dir {
                match entry {
                    Ok(entry) => {
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
                    Err(e) => {
                        return Err(io::Error::new(
                            io::ErrorKind::NotFound,
                            format!("Failed to list directory '{}': {}", path, e),
                        ))
                    }
                }
            }
        }
        Err(e) => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Failed to list directory '{}': {}", path, e),
            ))
        }
    }

    Ok(())
}

pub fn copy_directory(src: &str, dest: &str) -> Result<(), io::Error> {
    if !check_directory(src) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "Failed to copy directory '{}': Source is not a valid directory",
                src
            ),
        ));
    }

    match create_directory(dest) {
        Ok(_) => match list_directory(src, false, false) {
            Ok(entries) => {
                for entry in entries {
                    let src_path = format!("{}/{}", src, entry);
                    let dest_path = format!("{}/{}", dest, entry);

                    if check_file(&src_path) {
                        copy_file(&src_path, &dest_path)?;
                    } else {
                        copy_directory(&src_path, &dest_path)?;
                    }
                }

                Ok(())
            }
            Err(e) => Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Failed to copy directory '{}': {}", dest, e),
            )),
        },
        Err(e) => Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            format!("Failed to copy directory '{}': {}", dest, e),
        )),
    }
}
