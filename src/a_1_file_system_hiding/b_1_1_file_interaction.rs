// days_dvcs/src/a_1_file_system_hiding/b_1_1_file_interaction.rs
//
//! B.1.1 File Interaction
//! This component is responsible for reading, writing, appending, copying,
//! and deleting files.
//!
//! Parent Module: A.1 File System Hiding
//!
//! ## Usage:
//! The `read_file` function reads the content of a file and returns it as a string.
//!
//! The `read_struct` function reads the content of a file and deserializes it into a struct.
//!
//! The `write_file` function writes content to a file.
//!
//! The `write_struct` function serializes a struct and writes it to a file.
//!
//! The `append_file` function appends content to a file.
//!
//! The `copy_file` function copies a file from one location to another.
//!
//! The `delete_file` function deletes a file.
//!
//! ## Dependencies:
//! - none
//!
//! Author: Anakin (Yuesong Huang), Yifan (Alvin) Jiang
//! Date: 11/14/2024

use std::fs::{self, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use serde::de::DeserializeOwned;

pub fn check_file(path: &str) -> bool {
    Path::new(path).is_file()
}

pub fn get_filename(path: &str) -> String {
    Path::new(path)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_string()
}

pub fn get_parent(path: &str) -> String {
    Path::new(path)
        .parent()
        .unwrap_or_else(|| Path::new(""))
        .to_string_lossy()
        .into_owned()
}

pub fn get_relative_path(path: &str, base: &str, sanitize: bool) -> String {
    let relative_path = pathdiff::diff_paths(Path::new(&path), Path::new(&base))
        .unwrap_or_else(|| Path::new(path).to_path_buf());

    if sanitize {
        relative_path
            .components()
            .filter(|comp| matches!(comp, std::path::Component::Normal(_)))
            .collect::<PathBuf>()
            .to_string_lossy()
            .into_owned()
    } else {
        relative_path.to_string_lossy().into_owned()
    }
}

pub fn get_absolute_path(path: &str, base: &str) -> Result<String, io::Error> {
    Ok(Path::new(base)
        .join(path)
        .canonicalize()?
        .to_string_lossy()
        .into_owned())
}

pub fn read_file(path: &str) -> Result<String, io::Error> {
    let mut file = OpenOptions::new().read(true).open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

pub fn read_struct<T>(path: &str) -> Result<T, serde_json::Error>
where
    T: DeserializeOwned,
{
    let content = read_file(path).map_err(serde_json::Error::io)?;
    let result: T = serde_json::from_str(&content)?;
    Ok(result)
}

pub fn write_file(path: &str, content: &str) -> Result<(), io::Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true) // Ensure truncation
        .open(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn write_struct<T>(path: &str, s: &T) -> Result<(), serde_json::Error>
where
    T: serde::Serialize,
{
    let content = serde_json::to_string_pretty(s)?;
    write_file(path, &content).map_err(serde_json::Error::io)?;
    Ok(())
}

#[allow(dead_code)]
pub fn append_file(path: &str, content: &str) -> Result<(), io::Error> {
    let mut file = OpenOptions::new().write(true).append(true).open(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn copy_file(src: &str, dest: &str) -> Result<(), io::Error> {
    if Path::new(src).is_file() {
        fs::copy(src, dest)?;
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "File not found."))
    }
}

pub fn delete_file(path: &str) -> Result<(), io::Error> {
    if Path::new(path).is_file() {
        fs::remove_file(path)?;
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "File not found."))
    }
}
