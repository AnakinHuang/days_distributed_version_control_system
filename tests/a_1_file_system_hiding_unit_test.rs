// days_dvcs/tests/a_1_file_system_hiding_unit_test.rs
//
// Unit tests for the A.1 File System Hiding Module
// To run these tests, use: cargo test --test a_1_file_system_hiding_unit_test
//
// Author: Anakin (Yuesong Huang), Yifan (Alvin) Jiang
// Date: 11/14/2024

use days_dvcs::a_1_file_system_hiding::b_1_1_file_interaction::*;
use days_dvcs::a_1_file_system_hiding::b_1_2_directory_interaction::*;
use days_dvcs::a_1_file_system_hiding::b_1_3_metadata_management::*;

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;
    use std::time::SystemTime;
    use serde::{Deserialize, Serialize};

    /// B.1.1 File Interaction

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct TestStruct {
        key: String,
        value: i32,
    }

    #[test]
    fn test_check_file() {
        let path = "test_file.txt";
        fs::write(path, "test").unwrap();
        assert!(check_file(path));
        fs::remove_file(path).unwrap();
        assert!(!check_file(path));
    }
    
    #[test]
    fn test_read_file() {
        let path = "./src/test_files/a_1_file_system_hiding_unit_test_files/test_read.txt";
        let content = "Hello, world!";
        write_file(path, content).unwrap();

        let result = read_file(path).unwrap();
        assert_eq!(result, content);

        delete_file(path).unwrap();
    }

    #[test]
    fn test_read_struct() {
        let path = "test_struct.json";
        let test_data = TestStruct {
            key: "example".to_string(),
            value: 42,
        };
        let json = serde_json::to_string_pretty(&test_data).unwrap();
        fs::write(path, json).unwrap();

        let result: TestStruct = read_struct(path).unwrap();
        assert_eq!(result, test_data);

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_write_file() {
        let path = "./src/test_files/a_1_file_system_hiding_unit_test_files/test_write.txt";
        let content = "Hello, world!";
        write_file(path, content).unwrap();

        let result = read_file(path).unwrap();
        assert_eq!(result, content);

        delete_file(path).unwrap();
    }

    #[test]
    fn test_write_struct() {
        let path = "test_write_struct.json";
        let test_data = TestStruct {
            key: "example".to_string(),
            value: 42,
        };

        write_struct(path, &test_data).unwrap();
        let result: TestStruct = read_struct(path).unwrap();
        assert_eq!(result, test_data);

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_delete_file() {
        let path = "./src/test_files/a_1_file_system_hiding_unit_test_files/test_delete.txt";
        write_file(path, "Hello, world!").unwrap();
        delete_file(path).unwrap();

        let result = read_file(path);
        assert!(result.is_err());
    }

    #[test]
    fn test_append_file() {
        let path = "./src/test_files/a_1_file_system_hiding_unit_test_files/test_append.txt";
        write_file(path, "Hello").unwrap();
        append_file(path, ", world!").unwrap();

        let result = read_file(path).unwrap();
        assert_eq!(result, "Hello, world!");

        delete_file(path).unwrap();
    }
    
    #[test]
    fn test_copy_file() {
        let from = "./src/test_files/a_1_file_system_hiding_unit_test_files/test_copy_from.txt";
        let to = "./src/test_files/test_copy_to.txt";
        write_file(from, "Copy this").unwrap();
        copy_file(from, to).unwrap();

        let result = read_file(to).unwrap();
        assert_eq!(result, "Copy this");

        delete_file(from).unwrap();
        delete_file(to).unwrap();
    }
    
    /// B.1.2 Directory Interaction
    
    #[test]
    fn test_create_directory() {
        let path = "./test_dir_create";
        create_directory(path).unwrap();
        assert!(Path::new(path).exists());
        delete_directory(path, true).unwrap();
    }

    #[test]
    fn test_delete_directory_non_recursive() {
        let path = "./test_dir_delete_non_recursive";
        create_directory(path).unwrap();
        let result = delete_directory(path, false);
        assert!(result.is_ok());
        assert!(!Path::new(path).exists());
    }

    #[test]
    fn test_delete_directory_recursive() {
        let dir_path = "./test_dir_delete_recursive";
        let file_path = "./test_dir_delete_recursive/test.txt";

        create_directory(dir_path).unwrap();
        write_file(file_path, "Hello World").unwrap();
        let result = delete_directory(dir_path, true);
        assert!(result.is_ok());
        assert!(!Path::new(dir_path).exists());
    }

    #[test]
    fn test_list_directory() {
        let dir_path = "./test_dir_list";
        let file_path1 = "./test_dir_list/test1.txt";
        let file_path2 = "./test_dir_list/test2.txt";

        create_directory(dir_path).unwrap();
        write_file(file_path1, "test1.txt").unwrap();
        write_file(file_path2, "test2.txt").unwrap();

        let entries = list_directory(dir_path).unwrap();
        assert!(entries.contains(&"test1.txt".to_string()));
        assert!(entries.contains(&"test2.txt".to_string()));

        delete_directory(dir_path, true).unwrap();
    }

    #[test]
    fn test_copy_directory() {
        let src_dir = "test_src_dir";
        let dest_dir = "test_dest_dir";

        create_directory(src_dir).unwrap();
        let file_path = format!("{}/file.txt", src_dir);
        fs::write(&file_path, "test content").unwrap();

        copy_directory(src_dir, dest_dir).unwrap();

        assert!(check_file(&format!("{}/file.txt", dest_dir)));
        assert_eq!(fs::read_to_string(&format!("{}/file.txt", dest_dir)).unwrap(), "test content");

        delete_directory(src_dir, true).unwrap();
        delete_directory(dest_dir, true).unwrap();
    }
    
    /// B.1.3 Metadata Management

    #[test]
    fn test_get_file_metadata() {
        let path = "./test_metadata.txt";
        write_file(path, "").unwrap();

        let metadata = get_file_metadata(path).unwrap();
        assert_eq!(metadata.size, 0);
        assert!(metadata.last_modified <= SystemTime::now());
        assert!(!metadata.is_directory);

        delete_file(path).unwrap();
    }

    #[test]
    fn test_get_file_size() {
        let path = "./test_size.txt";
        let content = "Hello, world!";
        write_file(path, content).unwrap();

        let size = get_file_size(path).unwrap();
        assert_eq!(size, content.len() as u64);

        delete_file(path).unwrap();
    }

    #[test]
    fn test_get_last_modified() {
        let path = "test_last_modified.txt";
        write_file(path, "").unwrap();

        let last_modified = get_last_modified(path).unwrap();
        assert!(last_modified <= SystemTime::now());

        delete_file(path).unwrap();
    }
}
