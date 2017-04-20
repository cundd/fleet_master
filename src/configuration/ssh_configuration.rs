use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SshConfiguration {
    pub host: String,
    pub port: String,
    pub command: String,
    pub username: String,
    pub password: Option<String>,
    pub passphrase: Option<String>,
    pub private_key: Option<PathBuf>,
    pub public_key: Option<PathBuf>,
}

impl SshConfiguration {
    pub fn new_with_password<S>(
        host: S,
        port: S,
        command: S,
        username: S,
        password: S,
    ) -> SshConfiguration where S: Into<String> {
        SshConfiguration {
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

    pub fn new_empty() -> SshConfiguration {
        SshConfiguration {
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


#[cfg(test)]
mod tests {
    use super::*;


    // username:password@host:port command
    // username@host:port command
    // username:password@host command
    // username@host command
    // private_key+username@host:port command


    #[test]
    fn scan_uri_test() {
        //        SshConfiguration
//        let uri = SshUriScanner::scan_uri("username:password@host:port command").unwrap();
//        assert_eq! (uri, SshUri::new_with_password("host", "port", "command", "username", "password"));


//        assert_eq! (uri.host, "host");
//        assert_eq! (uri.port, "port");
//        assert_eq! (uri.command, "command");


        //
        //        let file_path = PathBuf::from(file!());
        //        let mut file_path_abs: PathBuf = fs::canonicalize(&file_path).unwrap();
        //        file_path_abs.pop();
        //        file_path_abs.pop();
        //        file_path_abs.pop();
        //
        //        let mut json_file_path = file_path_abs.clone();
        //        json_file_path.push("tests/protocol-test-0.1.0.json");
        //
        //        let information = scanner.get_information_for_uri(json_file_path.to_str().unwrap()).unwrap();
        //        assert_eq!("0.1.0", information.fleet.protocol);
        //        assert_eq!(56, information.packages.all.len());
        //
        //        let core: &Package = &information.packages.all["core"];
        //        assert_eq!(core.key, "core");
        //        assert_eq!(core.state, "active");
        //        assert_eq!(core.is_active(), true);
        //
        //        let recycler: &Package = &information.packages.all["recycler"];
        //        assert_eq!(recycler.key, "recycler");
        //        assert_eq!(recycler.state, "inactive");
        //        assert_eq!(recycler.is_active(), false);
    }
}
