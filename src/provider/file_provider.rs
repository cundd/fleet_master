use crate::error::Error;
use crate::information::*;
use std::fs;
use std::fs::File;
use std::path::PathBuf;

pub struct FileProvider;

impl FileProvider {
    #[allow(dead_code)]
    fn get_information_for_uri<S>(self, uri: S) -> Result<Information, Error>
    where
        S: Into<String>,
    {
        let absolute_file_path: PathBuf = match fs::canonicalize(uri.into()) {
            Ok(p) => p,
            Err(e) => return Err(Error::from_error(&e)),
        };

        let file = match File::open(absolute_file_path) {
            Ok(file) => file,
            Err(e) => return Err(Error::from_error(&e)),
        };

        let information: Information = match serde_json::from_reader(file) {
            Ok(information) => information,
            Err(e) => return Err(Error::from_error(&e)),
        };

        Ok(information)
    }
}

impl super::Provider for FileProvider {
    fn new() -> Self {
        FileProvider {}
    }
}

#[cfg(test)]
mod tests {
    use crate::information::Package;
    use crate::test_helpers;

    use super::*;

    #[test]
    fn get_information_for_uri_test() {
        let file_provider = FileProvider {};

        let json_file_path = test_helpers::get_test_resource_path("protocol-test-0.1.0.json");

        assert!(json_file_path.as_path().exists(), "{:?}", json_file_path);
        let information = file_provider
            .get_information_for_uri(json_file_path.to_str().unwrap())
            .unwrap();
        assert_eq!("0.1.0", information.fleet.protocol);
        assert_eq!(56, information.packages.len());

        let core: &Package = &information.packages["core"];
        assert_eq!(core.key, "core");
        assert_eq!(core.state, "active");
        assert!(core.is_active());

        let recycler: &Package = &information.packages["recycler"];
        assert_eq!(recycler.key, "recycler");
        assert_eq!(recycler.state, "inactive");
        assert!(!recycler.is_active());
    }
}
