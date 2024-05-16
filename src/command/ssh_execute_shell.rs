use crate::{
    configuration::{ConfigurationCollection, ConfigurationProvider},
    error::{Error, ErrorCollection},
    provider::{Provider, SshProvider},
    shell::ShellOutputCollection,
};
use std::path::PathBuf;

/// Execute a shell command for all hosts through SSH
pub fn execute_shell_for_collection(
    configuration_file: PathBuf,
    command: String,
) -> Result<(ShellOutputCollection, ErrorCollection), Error> {
    let configuration_collection = ConfigurationProvider::load(configuration_file.as_path())?;

    Ok(SshProvider::new().execute_shell_for_collection(command, configuration_collection))
}

/// Execute a shell command for the given hosts through SSH
pub fn execute_shell_for_hosts(
    configuration_file: PathBuf,
    hosts: &[String],
    command: String,
) -> Result<(ShellOutputCollection, ErrorCollection), Error> {
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
        Ok(SshProvider::new().execute_shell_for_collection(command, filtered))
    }
}
