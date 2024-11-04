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
    /// For simplicity, this is a local simulation.
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
        // Create the commit path in the repository
        let commit_path = Path::new(repo_directory).join(".dvcs").join("commits");
        FileSystemHiding::create_directory(commit_path.to_str().unwrap())?;

        // Read the content of the file to be committed
        let file_content = FileSystemHiding::read_file(file_directory)?;

        // Generate a unique name for the commit file
        let commit_file_path = commit_path.join(format!(
            "{}_{}.txt",
            Path::new(file_directory)
                .file_name()
                .unwrap()
                .to_string_lossy(),
            uuid::Uuid::new_v4()
        ));

        // Write the commit information to the file using FileSystemHiding
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

    /// Checks out a specific commit or branch, updating the working directory.
    pub fn checkout(directory: &str, target: &str) -> Result<(), RepoError> {
        let repo_path = Path::new(directory).join(".dvcs");
        if !repo_path.exists() {
            return Err(RepoError::RepoInitializationError(
                "No repository found".to_string(),
            ));
        }

        let commit_path = repo_path
            .join("commits")
            .join(format!("commit_{}.txt", target));
        if !commit_path.exists() {
            return Err(RepoError::CheckoutError(format!(
                "Commit or branch '{}' not found",
                target
            )));
        }

        // Simulate updating the working directory (details can vary depending on DVCS structure)
        println!("Switched to commit/branch: {}", target);
        Ok(())
    }

    /// Displays the status of the repository by listing tracked files.
    pub fn status(directory: &str) -> Result<String, RepoError> {
        let repo_path = Path::new(directory).join(".dvcs");
        if !repo_path.exists() {
            return Err(RepoError::RepoInitializationError(
                "No repository found".to_string(),
            ));
        }

        let mut status_info = String::from("Repository status:\n");
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

        Ok(status_info)
    }
}
