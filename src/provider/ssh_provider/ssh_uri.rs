use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub struct SshUri {
    pub host: String,
    pub port: String,
    pub command: String,
    pub username: String,
    pub password: Option<String>,
    pub passphrase: Option<String>,
    pub private_key: Option<PathBuf>,
    pub public_key: Option<PathBuf>,
}

impl SshUri {
    pub fn new_with_password<S>(
        host: S,
        port: S,
        command: S,
        username: S,
        password: S,
    ) -> Self where S: Into<String> {
        SshUri {
            host: host.into(),
            port: port.into(),
            command: command.into(),
            username: username.into(),
            password: Some(password.into()),
            passphrase: None,
            private_key: None,
            public_key: None,
        }
    }

    pub fn new_empty() -> Self {
        SshUri {
            host: "".to_owned(),
            port: "".to_owned(),
            command: "".to_owned(),
            username: "".to_owned(),
            password: None,
            passphrase: None,
            private_key: None,
            public_key: None,
        }
    }
}

// username:password@host:port command
// username@host:port command
// username:password@host command
// username@host command
// private_key+username@host:port command
