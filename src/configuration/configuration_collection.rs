use std::collections::HashMap;
use std::path::Path;

use crate::error::Error;
use super::Configuration;

pub type ConfigurationCollection = HashMap<String, Configuration>;

pub fn chunk_configuration_collection(collection: ConfigurationCollection, size: usize) -> Vec<ConfigurationCollection> {
    let list: Vec<_> = collection.into_iter().collect();

    list.chunks(size)
        .map(|c| c.iter().cloned().collect())
        .collect()
}

pub fn get_configuration_for_host(mut collection: ConfigurationCollection, host: &str, file: &Path) -> Result<Configuration, Error> {
    match collection.remove(host) {
        Some(configuration) => Ok(configuration),
        None => Err(Error::new(format!("Host {} not found in configuration file {}", host, file.to_str().unwrap())))
    }
}
