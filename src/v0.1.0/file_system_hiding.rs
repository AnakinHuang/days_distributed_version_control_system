use std::fs::{self, File, OpenOptions};
use std::io::{Error, Read, Write};

#[allow(dead_code)]
#[derive(Debug)]
pub enum FileError {
    IoError(Error),
    MetadataError(String),
}

impl From<Error> for FileError {
    fn from(err: Error) -> FileError {
        FileError::IoError(err)
    }
}

#[allow(dead_code)]
pub struct FileMetadata {
    pub size: u64,
    pub modified: String,
    pub permissions: String,
}

pub struct FileSystemHiding;

#[allow(dead_code)]
impl FileSystemHiding {
    /// Reads the content of a file and returns it as a String.
    pub fn read_file(path: &str) -> Result<String, FileError> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    /// Writes the provided content to the specified file.
    /// Creates a new file or overwrites if it already exists.
    pub fn write_file(path: &str, content: &str) -> Result<(), FileError> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    /// Creates a new directory at the given path.
    pub fn create_directory(path: &str) -> Result<(), FileError> {
        fs::create_dir_all(path)?;
        Ok(())
    }

    /// Deletes the specified file.
    pub fn delete_file(path: &str) -> Result<(), FileError> {
        fs::remove_file(path)?;
        Ok(())
    }

    /// Retrieves metadata for the specified file.
    pub fn get_file_metadata(path: &str) -> Result<FileMetadata, FileError> {
        let metadata = fs::metadata(path)?;
        let size = metadata.len();
        let modified = match metadata.modified() {
            Ok(time) => format!("{:?}", time),
            Err(_) => "Unknown modification time".to_string(),
        };
        let permissions = format!("{:?}", metadata.permissions());

        Ok(FileMetadata {
            size,
            modified,
            permissions,
        })
    }
}
