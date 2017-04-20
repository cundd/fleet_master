pub use super::Configuration;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::path::*;
use configuration::*;
use error::Error;
use serde_json;


pub struct ConfigurationProvider;

impl ConfigurationProvider {
    pub fn load(path: &Path) -> Result<ConfigurationCollection, Error> {
        let absolute_file_path: PathBuf = match fs::canonicalize(path) {
            Ok(p) => p,
            Err(e) => return Err(Error::new_from_error(e)),
        };

        let file = match File::open(absolute_file_path) {
            Ok(file) => file,
            Err(e) => return Err(Error::new_from_error(e)),
        };

        let configuration: ConfigurationCollection = match serde_json::from_reader(file) {
            Ok(configuration) => configuration,
            Err(e) => return Err(Error::new_from_error(e)),
        };

        Ok(configuration)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::path::Path;
    //    use configuration::;
    use test_helpers;

    #[test]
    fn load_test() {
        let json_file_path = test_helpers::get_test_resource_path("configuration-test-0.1.0.json");

        assert!(json_file_path.as_path().exists(), "{:?}", json_file_path);
        let configurations = ConfigurationProvider::load(json_file_path.as_path()).unwrap();

        assert_eq!(
        Configuration {
            host: "host".to_owned(),
            port: "port".to_owned(),
            command: "command".to_owned(),
            username: "username".to_owned(),
            password: Some("password".to_owned()),
            passphrase: Some("passphrase".to_owned()),
            private_key: Some(PathBuf::from("private_key".to_owned())),
            public_key: Some(PathBuf::from("public_key".to_owned()))
        },
        configurations["my.host.local"]
        );

        assert_eq!(
        Configuration {
            host: "host".to_owned(),
            port: "port".to_owned(),
            command: "command".to_owned(),
            username: "username".to_owned(),
            password: Some("password".to_owned()),
            passphrase: None,
            private_key: None,
            public_key: None
        },
        configurations["my.host-with-password.local"]
        );

        assert_eq!(
        Configuration {
            host: "host".to_owned(),
            port: "port".to_owned(),
            command: "command".to_owned(),
            username: "username".to_owned(),
            password: None,
            passphrase: Some("passphrase".to_owned()),
            private_key: Some(PathBuf::from("private_key".to_owned())),
            public_key: Some(PathBuf::from("public_key".to_owned()))
        },
        configurations["my.host-with-private_key.local"]
        );

        println!("{:?}", configurations["my.host.local"]);
        println!("{:?}", configurations["my.host-with-password.local"]);
        println!("{:?}", configurations["my.host-with-private_key.local"]);
        //        assert_eq!("0.1.0", configuration.);
        //        assert_eq!(56, configuration.packages.all.len());
        //
        //        let core: &Package = &configuration.packages.all["core"];
        //        assert_eq!(core.key, "core");
        //        assert_eq!(core.state, "active");
        //        assert_eq!(core.is_active(), true);
        //
        //        let recycler: &Package = &configuration.packages.all["recycler"];
        //        assert_eq!(recycler.key, "recycler");
        //        assert_eq!(recycler.state, "inactive");
        //        assert_eq!(recycler.is_active(), false);
    }
}
