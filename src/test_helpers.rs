//struct TestHelpers;
use std::path::PathBuf;
use std::fs;
use std::collections::HashMap;
use information::*;
use serde_json;

/// Returns the path to the testing resources
pub fn get_test_resources_path() -> PathBuf {
    let file_path = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "tests");
    match fs::canonicalize(&file_path) {
        Ok(f) => f,
        Err(e) => panic!("Error for file {}: {}", file!(), e),
    }
}

/// Returns the path to the testing resources
pub fn get_test_resource_path(resource_name: &str) -> PathBuf {
    let mut file_path = get_test_resources_path();

    for sub_path in resource_name.split("/") {
        file_path.push(sub_path);
    }

    file_path
}

/// Return [`Packages`] instance for testing
pub fn get_test_packages() -> Packages {
    let content = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/packages.json"));
    let packages: HashMap<String, Package> = match serde_json::from_str(&content) {
        Ok(collection) => collection,
        Err(e) => panic!("Could not deserialize test packages: {}", e),
    };

    Packages { all: packages }
}

#[allow(unused)]
pub struct ScopeCall<F: FnOnce()> {
    pub callback: Option<F>
}

impl<F: FnOnce()> Drop for ScopeCall<F> {
    fn drop(&mut self) {
        self.callback.take().unwrap()();
    }
}

#[macro_export]
macro_rules! cleanup {
    ($e:expr) => {
        let _scope_call = test_helpers::ScopeCall { callback: Some(|| -> () { $e; } )};
    };
}
