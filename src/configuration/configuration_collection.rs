use std::collections::HashMap;
use std::path::Path;

use error::Error;
use super::Configuration;

pub type ConfigurationCollection = HashMap<String, Configuration>;

pub fn get_configuration_for_host(mut collection: ConfigurationCollection, host: &str, file: &Path) -> Result<Configuration, Error> {
    match collection.remove(host) {
        Some(configuration) => Ok(configuration),
        None => Err(Error::new(format!("Host {} not found in configuration file {}", host, file.to_str().unwrap())))
    }
}
