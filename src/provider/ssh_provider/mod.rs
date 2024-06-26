mod ssh_connector;

use self::ssh_connector::SshConnector;
use crate::configuration::*;
use crate::error::*;
use crate::information::*;
use crate::shell::ShellOutputCollection;
use ssh2::Channel;
use ssh2::Session;
use std::io::prelude::*;
use std::net::TcpStream;
use std::sync::mpsc;
use std::thread;

pub struct SshProvider;

/// Fetch information from the server defined in `configuration`
fn fetch_information_through_ssh(configuration: &Configuration) -> Result<Information, Error> {
    let content = execute_shell_through_ssh(configuration.command(), configuration)?;
    let information: Information = match serde_json::from_str(&content) {
        Ok(information) => information,
        Err(e) => return Err(Error::with_error_and_details(&e, content)),
    };

    Ok(information)
}

/// Fetch information from the server defined in `configuration`
fn execute_shell_through_ssh<S: Into<String>>(
    command: S,
    configuration: &Configuration,
) -> Result<String, Error> {
    let address = format!("{}:{}", configuration.host(), configuration.port());
    let tcp = TcpStream::connect(address)?;
    let session: Session = SshConnector::new().connect(configuration, tcp)?;

    call_ssh_command(command.into(), &session)
}

fn call_ssh_command<S: Into<String>>(command: S, session: &Session) -> Result<String, Error> {
    let command_string: String = command.into();

    // Open channel
    let mut channel: Channel = session.channel_session()?;
    // Execute the command
    channel.exec(&command_string).unwrap();

    // Read the output
    let mut output = String::new();
    if let Err(e) = Read::read_to_string(&mut channel, &mut output) {
        return Err(Error::from_error(&e));
    }

    let exit_status = channel.exit_status()?;
    if exit_status == 0 && !output.is_empty() {
        return Ok(output);
    }

    let mut error_output = String::new();
    match Read::read_to_string(&mut channel.stderr(), &mut error_output) {
        Ok(_) => {
            let error_output_trimmed = error_output.trim();
            if !error_output_trimmed.is_empty() {
                Err(Error::new(error_output_trimmed))
            } else {
                Err(Error::new(output.trim()))
            }
        }
        Err(error) => Err(Error::from_error(&error)),
    }
}

impl SshProvider {
    /// Fetch information from the server defined in `configuration`
    pub fn get_information(&self, configuration: &Configuration) -> Result<Information, Error> {
        fetch_information_through_ssh(configuration)
    }

    /// Fetch the information for all hosts in the given configuration collection
    pub fn get_information_for_collection(
        &self,
        configuration_collection: ConfigurationCollection,
    ) -> (InformationCollection, ErrorCollection) {
        if configuration_collection.is_empty() {
            return (InformationCollection::new(), ErrorCollection::new());
        }
        if configuration_collection.len() <= self.get_number_of_threads() {
            return self.get_information_for_collection_sync(configuration_collection);
        }
        self.get_information_for_collection_async(configuration_collection)
    }

    /// Execute the given shell command for all hosts in the given configuration collection asynchronously
    pub fn execute_shell_for_collection(
        &self,
        command: String,
        configuration_collection: ConfigurationCollection,
    ) -> (ShellOutputCollection, ErrorCollection) {
        let mut error_collection = ErrorCollection::new();
        let mut output_collection = ShellOutputCollection::new();

        let (split_count, split_configuration_collection) =
            self.chunk_configration_collection_for_threads(configuration_collection);
        let (tx, rx) = mpsc::channel();

        for chunk in split_configuration_collection {
            let tx = tx.clone();
            let command_l = command.clone();

            thread::spawn(move || {
                let mut error_collection_l = ErrorCollection::new();
                let mut output_collection_l = ShellOutputCollection::new();

                for (host, configuration) in chunk {
                    match execute_shell_through_ssh(&command_l, &configuration) {
                        Ok(i) => {
                            let _ = output_collection_l.insert(host, i);
                        }
                        Err(e) => {
                            let _ = error_collection_l.insert(host, e);
                        }
                    };
                }

                tx.send((output_collection_l, error_collection_l)).unwrap();
            });
        }

        for _ in 0..split_count {
            let (output_collection_l, error_collection_l) = rx.recv().unwrap();
            error_collection.extend(error_collection_l);
            output_collection.extend(output_collection_l);
        }

        (output_collection, error_collection)
    }

    /// Execute the update command for all hosts in the given configuration collection asynchronously
    pub fn execute_update_for_collection(
        &self,
        configuration_collection: ConfigurationCollection,
    ) -> (ShellOutputCollection, ErrorCollection) {
        let mut error_collection = ErrorCollection::new();
        let mut output_collection = ShellOutputCollection::new();

        let filtered_collection: ConfigurationCollection = configuration_collection
            .into_iter()
            .filter(|(_, c)| c.update_command().is_some())
            .collect();

        let (split_count, split_configuration_collection) =
            self.chunk_configration_collection_for_threads(filtered_collection);
        let (tx, rx) = mpsc::channel();

        for chunk in split_configuration_collection {
            let tx = tx.clone();

            thread::spawn(move || {
                let mut error_collection_l = ErrorCollection::new();
                let mut output_collection_l = ShellOutputCollection::new();

                for (host, configuration) in chunk {
                    match execute_shell_through_ssh(
                        &configuration.update_command().unwrap(),
                        &configuration,
                    ) {
                        Ok(i) => {
                            let _ = output_collection_l.insert(host, i);
                        }
                        Err(e) => {
                            let _ = error_collection_l.insert(host, e);
                        }
                    };
                }

                tx.send((output_collection_l, error_collection_l)).unwrap();
            });
        }

        for _ in 0..split_count {
            let (output_collection_l, error_collection_l) = rx.recv().unwrap();
            error_collection.extend(error_collection_l);
            output_collection.extend(output_collection_l);
        }

        (output_collection, error_collection)
    }

    /// Fetch the information for all hosts in the given configuration collection synchronously
    fn get_information_for_collection_sync(
        &self,
        configuration_collection: ConfigurationCollection,
    ) -> (InformationCollection, ErrorCollection) {
        let mut error_collection = ErrorCollection::new();
        let mut information_collection = InformationCollection::new();

        for (host, configuration) in configuration_collection {
            match self.get_information(&configuration) {
                Ok(i) => {
                    let _ = information_collection.insert(host, i);
                }
                Err(e) => {
                    let _ = error_collection.insert(host, e);
                }
            };
        }

        (information_collection, error_collection)
    }

    /// Fetch the information for all hosts in the given configuration collection asynchronously
    fn get_information_for_collection_async(
        &self,
        configuration_collection: ConfigurationCollection,
    ) -> (InformationCollection, ErrorCollection) {
        let mut error_collection = ErrorCollection::new();
        let mut information_collection = InformationCollection::new();

        let (tx, rx) = mpsc::channel();

        // let number_of_threads = self.get_number_of_threads();
        // let size_of_chunk: usize =
        //     (configuration_collection.len() as f32 / number_of_threads as f32).ceil() as usize;
        // let split_configuration_collection: Vec<ConfigurationCollection> =
        //     chunk_configuration_collection(configuration_collection, size_of_chunk);
        // let split_count = split_configuration_collection.len();
        let (split_count, split_configuration_collection) =
            self.chunk_configration_collection_for_threads(configuration_collection);

        for chunk in split_configuration_collection {
            let tx = tx.clone();

            thread::spawn(move || {
                let mut error_collection_l = ErrorCollection::new();
                let mut information_collection_l = InformationCollection::new();

                for (host, configuration) in chunk {
                    match fetch_information_through_ssh(&configuration) {
                        Ok(i) => {
                            let _ = information_collection_l.insert(host, i);
                        }
                        Err(e) => {
                            let _ = error_collection_l.insert(host, e);
                        }
                    };
                }

                tx.send((information_collection_l, error_collection_l))
                    .unwrap();
            });
        }

        for _ in 0..split_count {
            let (information_collection_l, error_collection_l) = rx.recv().unwrap();
            error_collection.extend(error_collection_l);
            information_collection.extend(information_collection_l);
        }

        (information_collection, error_collection)
    }

    fn get_number_of_threads(&self) -> usize {
        4
    }

    fn chunk_configration_collection_for_threads(
        &self,
        configuration_collection: ConfigurationCollection,
    ) -> (usize, Vec<ConfigurationCollection>) {
        if configuration_collection.is_empty() {
            return (0, vec![]);
        }

        let number_of_threads = self.get_number_of_threads();
        let size_of_chunk: usize =
            (configuration_collection.len() as f32 / number_of_threads as f32).ceil() as usize;
        let split_configuration_collection: Vec<ConfigurationCollection> =
            chunk_configuration_collection(configuration_collection, size_of_chunk);
        let split_count = split_configuration_collection.len();

        (split_count, split_configuration_collection)
    }
}

impl super::Provider for SshProvider {
    fn new() -> Self {
        SshProvider {}
    }
}

#[cfg(test)]
mod tests {
    use crate::configuration::helper::Helper;
    use crate::configuration::Configuration;

    use super::*;

    #[test]
    fn get_information_for_uri_test() {
        let provider = SshProvider {};
        let configuration = Configuration::new_with_public_key(
            "not-a-host",
            22,
            "",
            "not-a-user",
            Helper::get_ssh_file_path("not-a-file"),
            None,
            None,
        );

        assert!(provider.get_information(&configuration).is_err());
    }
}
