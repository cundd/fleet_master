use provider::Provider;
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use information::*;
use serde_json;

pub struct FileProvider {}

impl Provider for FileProvider {
    fn get_information_for_uri<S>(self, uri: S) -> Result<Information, Error> where S: Into<String> {
        let absolute_file_path: PathBuf = fs::canonicalize(&uri.into()).unwrap();

        let file = match File::open(absolute_file_path) {
            Ok(file) => file,
            Err(e) => return Err(Error::new_from_error(e)),
        };

        let information: Information = match serde_json::from_reader(file) {
            Ok(information) => information,
            Err(e) => return Err(Error::new_from_error(e)),
        };

        Ok(information)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use information::Package;

    #[test]
    fn get_information_for_uri_test() {
        let file_provider = FileProvider {};

        let file_path = PathBuf::from(file!());
        let mut file_path_abs: PathBuf = fs::canonicalize(&file_path).unwrap();
        file_path_abs.pop();
        file_path_abs.pop();
        file_path_abs.pop();

        let mut json_file_path = file_path_abs.clone();
        json_file_path.push("tests/protocol-test-0.1.0.json");

        let information = file_provider.get_information_for_uri(json_file_path.to_str().unwrap()).unwrap();
        assert_eq!("0.1.0", information.fleet.protocol);
        assert_eq!(56, information.packages.all.len());

        let core: &Package = &information.packages.all["core"];
        assert_eq!(core.key, "core");
        assert_eq!(core.state, "active");
        assert_eq!(core.is_active(), true);

        let recycler: &Package = &information.packages.all["recycler"];
        assert_eq!(recycler.key, "recycler");
        assert_eq!(recycler.state, "inactive");
        assert_eq!(recycler.is_active(), false);
    }
}
