use error::Error;
use std::iter;
use std::str::Chars;

use super::ssh_uri::SshUri;

struct SshUriScanner;

impl SshUriScanner {
    pub fn scan_uri<S>(uri: S) -> Result<SshUri, Error> where S: Into<String> {
        let ssh_uri = SshUri::new_empty();
        let uri_string: String = uri.into();


        SshUriScanner::split(uri_string.chars().rev());

        Ok(ssh_uri)
    }


    fn split(input: iter::Rev<Chars>) -> Vec<String> {
        let mut tokens: Vec<String> = vec![];
        let mut word = String::from("");
        //        let mut locked = TokenizerLockMode::None;

        for c in input {}

        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::ssh_uri::SshUri;

    #[test]
    fn scan_uri_test() {
        let uri = SshUriScanner::scan_uri("username:password@host:port command").unwrap();
        assert_eq!(uri, SshUri::new_with_password("host", "port", "command", "username", "password"));


        assert_eq!(uri.host, "host");
        assert_eq!(uri.port, "port");
        assert_eq!(uri.command, "command");


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
        //        assert_eq!(56, information.packages.len());
        //
        //        let core: &Package = &information.packages["core"];
        //        assert_eq!(core.key, "core");
        //        assert_eq!(core.state, "active");
        //        assert_eq!(core.is_active(), true);
        //
        //        let recycler: &Package = &information.packages["recycler"];
        //        assert_eq!(recycler.key, "recycler");
        //        assert_eq!(recycler.state, "inactive");
        //        assert_eq!(recycler.is_active(), false);
    }
}
