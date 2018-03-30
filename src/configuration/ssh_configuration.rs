use std::path::*;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct SshConfiguration {
    host: String,
    port: u16,
    command: String,
    username: String,
    password: Option<String>,
    passphrase: Option<String>,
    private_key: Option<PathBuf>,
    public_key: Option<PathBuf>,
}

impl SshConfiguration {
    pub fn new<S: Into<String>>(
        host: S,
        port: u16,
        command: S,
        username: S,
        password: Option<S>,
        passphrase: Option<S>,
        private_key: Option<PathBuf>,
        public_key: Option<PathBuf>,
    ) -> Self {
        SshConfiguration {
            port,
            host: host.into(),
            command: command.into(),
            username: username.into(),
            password: password.map(|s| s.into()),
            passphrase: passphrase.map(|s| s.into()),
            private_key: private_key.into(),
            public_key: public_key.into(),
        }
    }
    pub fn new_with_password<S>(
        host: S,
        port: u16,
        command: S,
        username: S,
        password: S,
    ) -> Self where S: Into<String> {
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

    pub fn new_with_public_key<S>(
        host: S,
        port: u16,
        command: S,
        username: S,
        private_key: PathBuf,
        public_key: Option<PathBuf>,
        passphrase: Option<S>,
    ) -> Self where S: Into<String> {
        let passphrase_string = match passphrase {
            Some(p) => Some(p.into()),
            None => None,
        };
        SshConfiguration {
            port,
            host: host.into(),
            command: command.into(),
            username: username.into(),
            password: None,
            passphrase: passphrase_string,
            private_key: Some(private_key),
            public_key,
        }
    }


    //    pub fn new_with_public_key<S>(
    //        host: S,
    //        port: S,
    //        command: S,
    //        username: S,
    //        private_key: S,
    //        public_key: Option<S>,
    //        passphrase: Option<S>,
    //    ) -> Self where S: Into<String> {
    //        let passphrase_string = match passphrase {
    //            Some(p) => Some(p.into()),
    //            None => None,
    //        };
    //        let public_key_string = match public_key {
    //            Some(p) => Some(PathBuf::from(p.into())),
    //            None => None,
    //        };
    //        SshConfiguration {
    //            host: host.into(),
    //            port: port.into(),
    //            command: command.into(),
    //            username: username.into(),
    //            password: None,
    //            passphrase: passphrase_string,
    //            private_key: Some(PathBuf::from(private_key.into())),
    //            public_key: public_key_string,
    //        }
    //    }

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
        self.private_key.clone()
    }
    pub fn public_key(&self) -> Option<PathBuf> {
        self.public_key.clone()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use configuration::Helper;

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
            SshConfiguration::new_with_public_key("localhost", 22, "cmd", "daniel", private_key.clone(), None, None)
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
            SshConfiguration::new_with_public_key("localhost", 22, "cmd", "daniel", private_key.clone(), Some(public_key.clone()), None)
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
            SshConfiguration::new_with_public_key("localhost", 22, "cmd", "daniel", private_key.clone(), Some(public_key.clone()), Some("passphrase"))
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
}
