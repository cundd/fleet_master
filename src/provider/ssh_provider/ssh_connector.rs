use std::env;
use std::net::TcpStream;

use ssh2::Session;

use crate::configuration::Configuration;
use crate::error::*;

pub struct SshConnector {}

impl SshConnector {
    /// Create a new SSH connector
    pub fn new() -> Self {
        SshConnector {}
    }

    /// Establish a SSH connection with the given configuration
    pub fn connect(
        &self,
        configuration: &Configuration,
        tcp: &TcpStream,
    ) -> Result<Session, Error> {
        // Connect to the SSH server
        let mut session = Session::new().unwrap();
        session.handshake(&tcp).unwrap();

        if configuration.password().is_some() {
            return self.authenticate_password(configuration, session);
        } else if configuration.private_key().is_some() {
            return self.authenticate_public_key(configuration, session);
        }

        if session.authenticated() {
            Ok(session)
        } else {
            self.authenticate_agent(configuration, session)
        }
    }

    fn authenticate_password(
        &self,
        configuration: &Configuration,
        session: Session,
    ) -> Result<Session, Error> {
        let password = configuration.password().unwrap();
        session.userauth_password(&configuration.username(), &password)?;

        Ok(session)
    }

    fn authenticate_agent(
        &self,
        configuration: &Configuration,
        session: Session,
    ) -> Result<Session, Error> {
        session.userauth_agent(&configuration.username())?;

        Ok(session)
    }

    fn get_passphrase(&self, configuration: &Configuration) -> Option<String> {
        if let Some(ref passphrase) = configuration.passphrase() {
            return Some(passphrase.to_owned());
        }

        self.get_passphrase_from_env()
    }

    fn authenticate_public_key(
        &self,
        configuration: &Configuration,
        session: Session,
    ) -> Result<Session, Error> {
        let passphrase_option = self.get_passphrase(&configuration);
        let passphrase: Option<&str> = match passphrase_option {
            Some(ref val) => Some(&val),
            None => None,
        };

        let public_key = match configuration.public_key() {
            Some(p) => Some(p.clone()),
            None => None,
        };

        session.userauth_pubkey_file(
            &configuration.username(),
            match public_key {
                Some(ref p) => Some(p),
                None => None,
            },
            &configuration.private_key().as_ref().unwrap(),
            passphrase,
        )?;

        Ok(session)
    }

    fn get_passphrase_from_env<'a>(&self) -> Option<String> {
        let key = "PASSPHRASE";
        match env::var(key) {
            Ok(val) => Some(val),
            Err(e) => {
                println!("Couldn't get passphrase from env: {}", e);
                None
            }
        }
    }
}
