use std::path::*;

use dirs;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct SshConfiguration {
    host: String,

    #[serde(default = "default_port")]
    port: u16,
    command: String,
    username: String,
    password: Option<String>,
    passphrase: Option<String>,
    private_key: Option<PathBuf>,
    public_key: Option<PathBuf>,
}

fn default_port() -> u16 {
    22
}

impl SshConfiguration {
    #[allow(clippy::too_many_arguments)]
    pub fn new<S, P>(
        host: S,
        port: u16,
        command: S,
        username: S,
        password: Option<S>,
        passphrase: Option<S>,
        private_key: Option<P>,
        public_key: Option<P>,
    ) -> Self
    where
        S: Into<String>,
        P: AsRef<Path>,
    {
        SshConfiguration {
            port,
            host: host.into(),
            command: command.into(),
            username: username.into(),
            password: password.map(|s| s.into()),
            passphrase: passphrase.map(|s| s.into()),
            private_key: as_path_buf_option(private_key),
            public_key: as_path_buf_option(public_key),
        }
    }
    pub fn new_with_password<S>(host: S, port: u16, command: S, username: S, password: S) -> Self
    where
        S: Into<String>,
    {
        SshConfiguration {
            port,
            host: host.into(),
            command: command.into(),
            username: username.into(),
            password: Some(password.into()),
            passphrase: None,
            private_key: None,
            public_key: None,
        }
    }

    pub fn new_with_public_key<S, P>(
        host: S,
        port: u16,
        command: S,
        username: S,
        private_key: P,
        public_key: Option<P>,
        passphrase: Option<S>,
    ) -> Self
    where
        S: Into<String>,
        P: AsRef<Path>,
    {
        let passphrase_string = passphrase.map(|p| p.into());
        SshConfiguration {
            port,
            host: host.into(),
            command: command.into(),
            username: username.into(),
            password: None,
            passphrase: passphrase_string,
            private_key: Some(private_key.as_ref().to_path_buf()),
            public_key: as_path_buf_option(public_key),
        }
    }

    pub fn new_empty() -> Self {
        SshConfiguration {
            host: "".to_owned(),
            port: 0,
            command: "".to_owned(),
            username: "".to_owned(),
            password: None,
            passphrase: None,
            private_key: None,
            public_key: None,
        }
    }

    pub fn host(&self) -> &String {
        &self.host
    }
    pub fn port(&self) -> u16 {
        self.port
    }
    pub fn command(&self) -> &String {
        &self.command
    }
    pub fn username(&self) -> &String {
        &self.username
    }
    pub fn password(&self) -> Option<String> {
        self.password.clone()
    }
    pub fn passphrase(&self) -> Option<String> {
        self.passphrase.clone()
    }
    pub fn private_key(&self) -> Option<PathBuf> {
        match self.private_key {
            Some(ref p) => patch_key_path(p),
            None => None,
        }
    }
    pub fn public_key(&self) -> Option<PathBuf> {
        match self.public_key {
            Some(ref p) => patch_key_path(p),
            None => None,
        }
    }
}

fn as_path_buf_option<P: AsRef<Path>>(input: Option<P>) -> Option<PathBuf> {
    input.map(|p| p.as_ref().to_path_buf())
}

fn patch_key_path(p: &Path) -> Option<PathBuf> {
    if p.starts_with("~/") {
        let path_relative: String = p.to_string_lossy().chars().skip(2).collect();

        match dirs::home_dir() {
            Some(mut home) => {
                home.push(path_relative);

                Some(home)
            }
            None => None,
        }
    } else {
        Some(p.to_path_buf())
    }
}

#[cfg(test)]
mod tests {
    use crate::configuration::Helper;

    use super::*;

    // username:password@host:port command
    // username@host:port command
    // username:password@host command
    // username@host command
    // private_key+username@host:port command

    #[test]
    fn new_with_public_key_test() {
        let private_key = Helper::get_ssh_file_path("id_rsa");
        let public_key = Helper::get_ssh_file_path("id_rsa.pub");

        assert_eq!(
            SshConfiguration {
                host: "localhost".to_owned(),
                port: 22,
                command: "cmd".to_owned(),
                username: "daniel".to_owned(),
                password: None,
                passphrase: None,
                private_key: Some(private_key.clone()),
                public_key: None,
            },
            SshConfiguration::new_with_public_key(
                "localhost",
                22,
                "cmd",
                "daniel",
                private_key.clone(),
                None,
                None
            )
        );

        assert_eq!(
            SshConfiguration {
                host: "localhost".to_owned(),
                port: 22,
                command: "cmd".to_owned(),
                username: "daniel".to_owned(),
                password: None,
                passphrase: None,
                private_key: Some(private_key.clone()),
                public_key: Some(public_key.clone()),
            },
            SshConfiguration::new_with_public_key(
                "localhost",
                22,
                "cmd",
                "daniel",
                private_key.clone(),
                Some(public_key.clone()),
                None
            )
        );

        assert_eq!(
            SshConfiguration {
                host: "localhost".to_owned(),
                port: 22,
                command: "cmd".to_owned(),
                username: "daniel".to_owned(),
                password: None,
                passphrase: Some("passphrase".to_owned()),
                private_key: Some(private_key.clone()),
                public_key: Some(public_key.clone()),
            },
            SshConfiguration::new_with_public_key(
                "localhost",
                22,
                "cmd",
                "daniel",
                private_key,
                Some(public_key),
                Some("passphrase")
            )
        );
    }

    #[test]
    fn new_with_password_test() {
        assert_eq!(
            SshConfiguration {
                host: "localhost".to_owned(),
                port: 22,
                command: "cmd".to_owned(),
                username: "daniel".to_owned(),
                password: Some("password".to_owned()),
                passphrase: None,
                private_key: None,
                public_key: None,
            },
            SshConfiguration::new_with_password("localhost", 22, "cmd", "daniel", "password")
        );
    }

    #[test]
    fn new_empty_test() {
        assert_eq!(
            SshConfiguration {
                host: "".to_owned(),
                port: 0,
                command: "".to_owned(),
                username: "".to_owned(),
                password: None,
                passphrase: None,
                private_key: None,
                public_key: None,
            },
            SshConfiguration::new_empty()
        );
    }

    #[test]
    fn private_key_test() {
        let c = SshConfiguration::new_with_public_key(
            "localhost",
            22,
            "cmd",
            "daniel",
            "~/.ssh/my_key",
            Some("~/.ssh/my_key.pub"),
            None,
        );

        assert_eq!(
            format!("{}/{}", Helper::get_ssh_dir().to_string_lossy(), "my_key"),
            c.private_key().unwrap().to_string_lossy()
        );
        assert_eq!(
            format!(
                "{}/{}",
                Helper::get_ssh_dir().to_string_lossy(),
                "my_key.pub"
            ),
            c.public_key().unwrap().to_string_lossy()
        );
    }
}
