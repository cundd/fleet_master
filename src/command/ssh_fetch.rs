use crate::{
    configuration::{Configuration, ConfigurationCollection, ConfigurationProvider},
    error::{Error, ErrorCollection},
    information::{CollectionResult, Information, InformationCollection},
    provider::{Provider, SshProvider},
};
use std::path::PathBuf;

/// Fetch information for the given host
///
/// This will perform the following steps:
///     1. Look for the configuration file
///     2. Fetch the configuration for the host
///     3. Fetch the information using the configuration
pub fn fetch_information_for_host(
    configuration_file: PathBuf,
    host: &str,
) -> Result<Information, Error> {
    let configuration =
        ConfigurationProvider::get_configuration_for_host(configuration_file.as_path(), host)?;

    fetch_information(&configuration)
}

/// Fetch information for the given hosts
///
/// This will perform the following steps:
///     1. Look for the configuration file
///     2. Filter the list of configurations
///     3. Fetch the information using the configurations
pub fn fetch_information_for_hosts(
    configuration_file: PathBuf,
    hosts: &[String],
) -> CollectionResult {
    let configuration_collection = ConfigurationProvider::load(configuration_file.as_path())?;
    let filtered: ConfigurationCollection = configuration_collection
        .into_iter()
        .filter(|(host, _)| !host.is_empty() && hosts.contains(host))
        .collect();

    if filtered.is_empty() {
        Err(Error::new(format!(
            "{}: {}",
            if hosts.len() > 1 {
                "No configurations found for hosts"
            } else {
                "No configuration found for host"
            },
            hosts.join(", ")
        )))
    } else {
        Ok(fetch_information_for_configuration_collection(filtered))
    }
}

/// Fetch the information for all hosts in the configuration collection
pub fn fetch_information_collection(configuration_file: PathBuf) -> CollectionResult {
    let configuration_collection = match ConfigurationProvider::load(configuration_file.as_path()) {
        Ok(c) => c,
        Err(e) => {
            return Err(Error::new(format!(
                "Error when loading configuration file '{}': {}",
                configuration_file.to_string_lossy(),
                e
            )))
        }
    };

    Ok(fetch_information_for_configuration_collection(
        configuration_collection,
    ))
}

/// Fetch the information for all hosts in the given configuration collection
fn fetch_information_for_configuration_collection(
    configuration_collection: ConfigurationCollection,
) -> (InformationCollection, ErrorCollection) {
    SshProvider::new().get_information_for_collection(configuration_collection)
}

/// Fetch information from the host in the given configuration
fn fetch_information(configuration: &Configuration) -> Result<Information, Error> {
    SshProvider::new().get_information(configuration)
}
