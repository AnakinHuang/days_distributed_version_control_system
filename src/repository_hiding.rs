use crate::file_system_hiding::{FileError, FileSystemHiding};
use std::fs;
use std::io::Error;
use std::path::Path;

#[allow(dead_code)]
#[derive(Debug)]
pub enum RepoError {
    IoError(Error),
    FileSystemError(FileError),
    RepoInitializationError(String),
    CommitError(String),
    CheckoutError(String),
}

impl From<Error> for RepoError {
    fn from(err: Error) -> RepoError {
        RepoError::IoError(err)
    }
}

impl From<FileError> for RepoError {
    fn from(err: FileError) -> RepoError {
        RepoError::FileSystemError(err)
    }
}

pub struct Repository;

#[allow(dead_code)]
impl Repository {
    /// Initializes a new repository by creating the necessary directory structure.
    pub fn init(repo_directory: &str) -> Result<(), RepoError> {
        let repo_path = Path::new(repo_directory).join(".dvcs");
        if repo_path.exists() {
            return Err(RepoError::RepoInitializationError(
                "Repository already exists".to_string(),
            ));
        }
        FileSystemHiding::create_directory(repo_path.to_str().unwrap())?;
        println!("Initialized empty DVCS repository in {}", repo_directory);
        Ok(())
    }

    /// Clones an existing repository from a given remote URL to a local directory.
    pub fn clone(remote_url: &str, directory: &str) -> Result<(), RepoError> {
        let source_path = Path::new(remote_url);
        let target_path = Path::new(directory);

        if !source_path.exists() || !source_path.is_dir() {
            return Err(RepoError::RepoInitializationError(
                "Remote repository not found".to_string(),
            ));
        }

        fs::create_dir_all(target_path)?;
        fs::copy(source_path, target_path)?;

        println!("Cloned repository from {} to {}", remote_url, directory);
        Ok(())
    }

    /// Commits a file from `file_directory` to the repository in `repo_directory` with a given message.
    pub fn commit(
        repo_directory: &str,
        file_directory: &str,
        message: &str,
    ) -> Result<(), RepoError> {
        let commit_path = Path::new(repo_directory).join(".dvcs").join("commits");
        FileSystemHiding::create_directory(commit_path.to_str().unwrap())?;

        let file_content = FileSystemHiding::read_file(file_directory)?;

        let commit_file_path = commit_path.join(format!(
            "{}_{}.{}",
            Path::new(file_directory)
                .file_stem()
                .unwrap()
                .to_string_lossy(),
            uuid::Uuid::new_v4(),
            Path::new(file_directory)
                .extension()
                .unwrap()
                .to_string_lossy()
        ));

        let commit_data = format!(
            "Commit message: {}\nTimestamp: {:?}\n\nFile Content:\n{}",
            message,
            chrono::Utc::now(),
            file_content
        );
        FileSystemHiding::write_file(commit_file_path.to_str().unwrap(), &commit_data)?;

        println!(
            "Committed changes from '{}' to '{}', with message: '{}'",
            file_directory,
            commit_file_path.to_string_lossy(),
            message
        );
        Ok(())
    }

    /// Checks out a specific commit by restoring the content to a file in the working directory.
    pub fn checkout(directory: &str, commit_id: &str) -> Result<String, RepoError> {
        let repo_path = Path::new(directory).join(".dvcs");
        if !repo_path.exists() {
            return Err(RepoError::RepoInitializationError(
                "No repository found".to_string(),
            ));
        }

        let commit_path_buf = repo_path.join("commits").join(format!("*_{}.*", commit_id));
        let commit_path = match glob::glob(commit_path_buf.to_str().unwrap()).ok().and_then(|mut paths| paths.next()) {
            Some(Ok(path)) => path,
            _ => {
                return Err(RepoError::CheckoutError(format!(
                    "Commit with UUID '{}' not found",
                    commit_id
                )));
            }
        };

        let commit_content = FileSystemHiding::read_file(commit_path.to_str().unwrap())?;

        let content_marker = "\n\nFile Content:\n";
        if let Some(content_start) = commit_content.find(content_marker) {
            let file_content = &commit_content[content_start + content_marker.len()..];
            let target_file_path = Path::new(directory).join("restored_file.txt");
            FileSystemHiding::write_file(target_file_path.to_str().unwrap(), file_content)?;

            Ok(format!("Checked out to commit '{}'. The content has been restored to '{}'.",
                          commit_path.file_name().unwrap().to_string_lossy(),
                          target_file_path.display()))
        } else {
            Err(RepoError::CheckoutError(
                "Commit file format is incorrect or content marker not found.".to_string(),
            ))
        }
    }

    /// Displays the status of the repository by listing tracked files and recent commits.
    pub fn status(directory: &str) -> Result<String, RepoError> {
        let repo_path = Path::new(directory).join(".dvcs");
        if !repo_path.exists() {
            return Err(RepoError::RepoInitializationError(
                "No repository found".to_string(),
            ));
        }

        let mut status_info = String::new();

        let entries = fs::read_dir(directory)?;
        for entry in entries {
            let entry = entry?;
            if entry.path().is_file() {
                status_info.push_str(&format!(
                    "Tracked file: {}\n",
                    entry.file_name().to_string_lossy()
                ));
            }
        }

        let commit_path = repo_path.join("commits");
        if commit_path.exists() {
            let commit_entries = fs::read_dir(commit_path)?;
            for commit_entry in commit_entries {
                let commit_entry = commit_entry?;
                if commit_entry.path().is_file() {
                    let commit_content = FileSystemHiding::read_file(commit_entry.path().to_str().unwrap())?;
                    status_info.push_str(&format!(
                        "# Commit: {}\n{}",
                        commit_entry.file_name().to_string_lossy(),
                        commit_content.lines().take(2)
                            .fold(String::new(), |acc, line| acc + "- " + line + "\n")
                    ));
                }
            }
        } else {
            status_info.push_str("No commits found.\n");
        }

        Ok(status_info)
    }
}
