//struct TestHelpers;
use std::path::PathBuf;
use std::fs;

/// Returns the path to the testing resources
pub fn get_test_resources_path() -> PathBuf {
    let file_path = PathBuf::from(file!());
    let mut file_path_abs: PathBuf = fs::canonicalize(&file_path).unwrap();
    file_path_abs.pop();
    file_path_abs.pop();
    file_path_abs.push("tests");

    file_path_abs
}

/// Returns the path to the testing resources
pub fn get_test_resource_path(resource_name: &str) -> PathBuf {
    let mut file_path = get_test_resources_path();
    file_path.push(resource_name);

    file_path
}
