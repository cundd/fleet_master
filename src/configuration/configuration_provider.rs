use std::fs::File;
use std::path::*;
use std::ffi::OsStr;
use configuration::*;
use error::Error;
use serde_json;

#[cfg(feature = "yaml")]
use serde_yaml;


pub struct ConfigurationProvider;

impl ConfigurationProvider {
    pub fn get_configuration_for_host(path: &Path, host: &str) -> Result<Configuration, Error> {
        let collection = Self::load(path)?;

        get_configuration_for_host(collection, host, path)
    }

    pub fn load(path: &Path) -> Result<ConfigurationCollection, Error> {
        let file = match File::open(path) {
            Ok(file) => file,
            Err(e) => return Err(Error::with_error_and_details(&e, e.to_string()))
        };

        if let Some(extension) = path.extension() {
            match extension.to_string_lossy().to_string().as_ref() {
                #[cfg(feature = "yaml")]
                "yaml" => ConfigurationProvider::load_yaml(file),
                "json" => ConfigurationProvider::load_json(file),
                _ => Err(build_file_format_error(extension))
            }
        } else {
            Err(Error::new(
                format!(
                    "Could not load configuration from '{}'",
                    path.to_string_lossy()
                )
            ))
        }
    }

    fn load_json(file: File) -> Result<ConfigurationCollection, Error> {
        match serde_json::from_reader(file) {
            Ok(configuration) => Ok(configuration),
            Err(e) => Err(Error::with_error_and_details(&e, e.to_string())),
        }
    }

    #[cfg(feature = "yaml")]
    fn load_yaml(file: File) -> Result<ConfigurationCollection, Error> {
        match serde_yaml::from_reader(file) {
            Ok(configuration) => Ok(configuration),
            Err(e) => Err(Error::with_error_and_details(&e, e.to_string())),
        }
    }
}

fn build_file_format_error(extension: &OsStr) -> Error {
    Error::new(
        format!(
            "Could not load configuration from file with extension '{}'",
            extension.to_string_lossy()
        )
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use test_helpers;

    fn assert_configuration(configurations: ConfigurationCollection) {
        assert_eq!(
            Configuration::new(
                "host",
                22,
                "command",
                "username",
                Some("password"),
                Some("passphrase"),
                Some(PathBuf::from("private_key".to_owned())),
                Some(PathBuf::from("public_key".to_owned())),
            ),
            configurations["my.host.local"]
        );

        assert_eq!(
            Configuration::new(
                "host",
                22,
                "command",
                "username",
                Some("password"),
                None,
                None::<&str>,
                None::<&str>,
            ),
            configurations["my.host-with-password.local"]
        );

        assert_eq!(
            Configuration::new(
                "host",
                22,
                "command",
                "username",
                None,
                Some("passphrase"),
                Some(PathBuf::from("private_key".to_owned())),
                Some(PathBuf::from("public_key".to_owned())),
            ),
            configurations["my.host-with-private_key.local"]
        );
    }

    #[test]
    fn load_json_test() {
        let json_file_path = test_helpers::get_test_resource_path("configuration-test-0.2.0.json");
        let configurations = ConfigurationProvider::load(json_file_path.as_path());
        assert!(configurations.is_ok(), "{:?}", configurations.unwrap_err());
        assert_configuration(configurations.unwrap());
    }

    #[test]
    #[cfg(yaml)]
    fn load_yaml_test() {
        let yaml_file_path = test_helpers::get_test_resource_path("configuration-test-0.2.0.yaml");
        let configurations = ConfigurationProvider::load(yaml_file_path.as_path()).unwrap();
        assert_configuration(configurations);
    }
}
