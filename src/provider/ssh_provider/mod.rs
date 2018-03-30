mod ssh_connector;

use std::thread;
use std::sync::mpsc;
use std::io::prelude::*;
use std::net::TcpStream;
use serde_json;
use ssh2::Session;
use ssh2::Channel;

use error::*;
use information::*;
use configuration::*;
use self::ssh_connector::SshConnector;

pub struct SshProvider;

/// Fetch information from the server given in the configuration
fn fetch_information_through_ssh(configuration: &Configuration) -> Result<Information, Error> {
    let address = format!("{}:{}", configuration.host, configuration.port);
    let tcp = match TcpStream::connect(&address) {
        Ok(t) => t,
        Err(e) => return Err(Error::from_error(&e)),
    };

    let session: Session = SshConnector::new().connect(&configuration, &tcp)?;

    let mut command = String::new();
    command.push_str(&configuration.command);

    let content = call_ssh_command(command, &session)?;
    let information: Information = match serde_json::from_str(&content) {
        Ok(information) => information,
        Err(e) => return Err(Error::with_error_and_details(&e, content)),
    };

    Ok(information)
}

fn call_ssh_command<S: Into<String>>(command: S, session: &Session) -> Result<String, Error> where S: Into<String> {
    let command_string: String = command.into();

    // Open channel
    let mut channel: Channel = match session.channel_session() {
        Ok(c) => c,
        Err(e) => return Err(Error::from_error(&e))
    };

    // Execute the command
    channel.exec(&command_string).unwrap();

    // Read the output
    let mut output = String::new();
    if let Err(e) = Read::read_to_string(&mut channel, &mut output) {
        return Err(Error::from_error(&e));
    }

    let exit_status = match channel.exit_status() {
        Ok(exit_status) => exit_status,
        Err(e) => return Err(Error::from_error(&e))
    };

    if exit_status == 0 {
        return Ok(output)
    }

    let mut error_output = String::new();
    match Read::read_to_string(&mut channel.stderr(), &mut error_output) {
        Ok(_) => return Err(Error::new(error_output.trim())),
        Err(error) => Err(Error::from_error(&error)),
    }
}


impl SshProvider {
    /// Fetch information from the server given in the configuration
    pub fn get_information(&self, configuration: &Configuration) -> Result<Information, Error> {
        fetch_information_through_ssh(configuration)
    }

    /// Fetch the information for all hosts in the given configuration collection
    pub fn get_information_for_collection(&self, configuration_collection: ConfigurationCollection) -> (InformationCollection, ErrorCollection) {
        if configuration_collection.len() == 0 {
            return (InformationCollection::new(), ErrorCollection::new());
        }
        if configuration_collection.len() <= self.get_number_of_threads() {
            return self.get_information_for_collection_sync(configuration_collection);
        }
        self.get_information_for_collection_async(configuration_collection)
    }

    /// Fetch the information for all hosts in the given configuration collection synchronously
    fn get_information_for_collection_sync(&self, configuration_collection: ConfigurationCollection) -> (InformationCollection, ErrorCollection) {
        let mut error_collection = ErrorCollection::new();
        let mut information_collection = InformationCollection::new();

        for (host, configuration) in configuration_collection {
            match self.get_information(&configuration) {
                Ok(i) => { let _ = information_collection.insert(host, i); }
                Err(e) => { let _ = error_collection.insert(host, e); }
            };
        }

        (information_collection, error_collection)
    }

    /// Fetch the information for all hosts in the given configuration collection asynchronously
    fn get_information_for_collection_async(&self, configuration_collection: ConfigurationCollection) -> (InformationCollection, ErrorCollection) {
        let mut error_collection = ErrorCollection::new();
        let mut information_collection = InformationCollection::new();

        let number_of_threads = self.get_number_of_threads();


        let (tx, rx) = mpsc::channel();

        let size_of_chunk: usize = (configuration_collection.len() as f32 / number_of_threads as f32).ceil() as usize;
        let split_configuration_collection: Vec<ConfigurationCollection> = chunk_configuration_collection(configuration_collection, size_of_chunk);
        let split_count = split_configuration_collection.len();

        for chunk in split_configuration_collection {
            let tx = tx.clone();

            thread::spawn(move || {
                let mut error_collection_l = ErrorCollection::new();
                let mut information_collection_l = InformationCollection::new();

                for (host, configuration) in chunk {
                    match fetch_information_through_ssh(&configuration) {
                        Ok(i) => { let _ = information_collection_l.insert(host, i); }
                        Err(e) => { let _ = error_collection_l.insert(host, e); }
                    };
                }

                tx.send((information_collection_l, error_collection_l)).unwrap();
            });
        }

        for _ in 0..split_count{
            let (information_collection_l, error_collection_l) = rx.recv().unwrap();
            error_collection.extend(error_collection_l);
            information_collection.extend(information_collection_l);
        }

        (information_collection, error_collection)
    }

    fn get_number_of_threads(&self) -> usize {
        return 4
    }
}


impl super::Provider for SshProvider {
    fn new() -> Self {
        SshProvider {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use configuration::Configuration;
    use configuration::Helper;

    #[test]
    fn get_information_for_uri_test() {
        let provider = SshProvider {};
        let configuration = Configuration::new_with_public_key(
            "not-a-host",
            "22",
            "",
            "not-a-user",
            Helper::get_ssh_file_path("not-a-file"),
            None,
            None
        );

        assert!(provider.get_information(&configuration).is_err());
    }
}
