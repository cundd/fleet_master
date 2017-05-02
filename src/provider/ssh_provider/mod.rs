mod ssh_connector;

use error::Error;
use information::*;
use configuration::*;
use self::ssh_connector::SshConnector;
use std::io::prelude::*;
use serde_json;
use ssh2::Session;
use ssh2::Channel;
use std::net::TcpStream;

pub struct SshProvider;

impl SshProvider {
    pub fn get_information(self, configuration: &Configuration) -> Result<Information, Error> {
        let address = format!("{}:{}", configuration.host, configuration.port);
        let tcp = match TcpStream::connect(&address) {
            Ok(t) => t,
            Err(e) => return Err(Error::new_from_error(e)),
        };

        let session: Session = SshConnector::new().connect(&configuration, &tcp)?;

        let mut command = String::new();
        command.push_str(&configuration.command);

        let content = self.call_ssh_command(command, &session)?;
        let information: Information = match serde_json::from_str(&content) {
            Ok(information) => information,
            Err(e) => return Err(Error::with_error_and_details(e, content)),
        };

        Ok(information)
    }


    fn call_ssh_command<S: Into<String>>(&self, command: S, session: &Session) -> Result<String, Error> where S: Into<String> {
        let command_string: String = command.into();

        // Open channel
        let mut channel: Channel = match session.channel_session() {
            Ok(c) => c,
            Err(e) => return Err(Error::new_from_error(e))
        };

        // Execute the command
        channel.exec(&command_string).unwrap();

        // Read the output
        let mut output = String::new();
        if let Err(e) = Read::read_to_string(&mut channel, &mut output) {
            return Err(Error::new_from_error(e));
        }

        // println!("'{}'", output);
        // println!("{}", channel.exit_status().unwrap());

        Ok(output)
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
