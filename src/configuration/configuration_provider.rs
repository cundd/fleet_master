pub use super::Configuration;
use std::fs;
use std::fs::File;
use std::path::*;
use configuration::*;
use error::Error;
use serde_json;
use serde_yaml;


pub struct ConfigurationProvider;

impl ConfigurationProvider {
    pub fn load(path: &Path) -> Result<ConfigurationCollection, Error> {
        let absolute_file_path: PathBuf = match fs::canonicalize(path) {
            Ok(p) => p,
            Err(e) => return Err(Error::new_from_error(e)),
        };

        let file = match File::open(absolute_file_path.as_path()) {
            Ok(file) => file,
            Err(e) => return Err(Error::new_from_error(e)),
        };

        if let Some(extension) = absolute_file_path.as_path().extension() {
            if extension == "yaml" {
                return ConfigurationProvider::load_yaml(file);
            }
            if extension == "json" {
                return ConfigurationProvider::load_json(file);
            }

            return Err(Error::new(
                format!(
                    "Could not load configuration from file with extension '{}'",
                    extension.to_string_lossy()
                )
            ));
        }

        Err(Error::new(
            format!(
                "Could not load configuration from '{}'",
                absolute_file_path.to_string_lossy()
            )
        ))
    }

    fn load_json(file: File) -> Result<ConfigurationCollection, Error> {
        let configuration: ConfigurationCollection = match serde_json::from_reader(file) {
            Ok(configuration) => configuration,
            Err(e) => return Err(Error::new_from_error(e)),
        };

        Ok(configuration)
    }

    fn load_yaml(file: File) -> Result<ConfigurationCollection, Error> {
        let configuration: ConfigurationCollection = match serde_yaml::from_reader(file) {
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
    use test_helpers;

    fn assert_configuration(configurations: ConfigurationCollection) {
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
    }

    #[test]
    fn load_json_test() {
        let json_file_path = test_helpers::get_test_resource_path("configuration-test-0.1.0.json");
        let configurations = ConfigurationProvider::load(json_file_path.as_path()).unwrap();
        assert_configuration(configurations);
    }

    #[test]
    fn load_yaml_test() {
        let yaml_file_path = test_helpers::get_test_resource_path("configuration-test-0.1.0.yaml");
        let configurations = ConfigurationProvider::load(yaml_file_path.as_path()).unwrap();
        assert_configuration(configurations);
    }
}
