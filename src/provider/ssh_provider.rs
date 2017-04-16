use std::io::prelude::*;
use provider::Provider;
use ssh2::Session;
use ssh2::Channel;
use information::*;
use std::net::TcpStream;
use std::env;
use std::path::PathBuf;

pub struct SshProvider;

fn get_ssh_dir() -> PathBuf {
    let mut path = env::home_dir().unwrap();
    path.push(".ssh");

    return path
}


impl SshProvider {
    fn get_pubkey_passphrase(&self) -> Option<String> {
        let key = "PASSPHRASE";
        match env::var(key) {
            Ok(val) => Some(val),
            Err(e) => {
                println!("Couldn't get passphrase from env: {}", e);
                None
            }
        }
    }

    #[allow(unused_variables)]
    fn get_host_from_uri(&self, uri: &String) -> String {
        "127.0.0.1:22".to_string()
    }

    #[allow(unused_variables)]
    fn get_user_from_uri(&self, uri: &String) -> String {
        "daniel".to_string()
    }

    fn get_public_file_path(&self) -> PathBuf {
        let mut file_path = get_ssh_dir();
        file_path.push("id_rsa.pub");

        file_path
    }

    fn get_private_file_path(&self) -> PathBuf {
        let mut file_path = get_ssh_dir();
        file_path.push("id_rsa");

        file_path
    }

    fn connect(&self, uri: &String, tcp: &TcpStream) -> Result<Session, Error> {
        // Connect to the SSH server
        let mut session = Session::new().unwrap();
        session.handshake(&tcp).unwrap();

        let passphrase_result = self.get_pubkey_passphrase();

        let passphrase: Option<&str> = match passphrase_result {
            Some(ref val) => Some(&val),
            None => None,
        };

        // session.userauth_password(&self.get_user_from_uri(&uri), "").unwrap();
        if let Err(e) = session.userauth_pubkey_file(
            &self.get_user_from_uri(&uri),
            Some(self.get_public_file_path().as_path()),
            self.get_private_file_path().as_path(),
            passphrase
        ) {
            return Err(Error::new_from_error(e));
        }

        if !session.authenticated() {
            return Err(Error::new("Could not authenticate"));
        }

        Ok(session)
    }

    fn call_ssh_command<S: Into<String>>(&self, command: S, session: &Session) -> (i32, String) where S: Into<String> {
        let command_string: String = command.into();

        // Open channel
        let mut channel: Channel = session.channel_session().unwrap();
        channel.exec(&command_string).unwrap();

        let mut output = String::new();
        if let Err(_) = Read::read_to_string(&mut channel, &mut output) {
            return (
                channel.exit_status().unwrap(),
                "".to_string()
            );
        }

        println!("'{}'", output);
        println!("{}", channel.exit_status().unwrap());

        (
            channel.exit_status().unwrap(),
            output
        )
    }
}

impl Provider for SshProvider {
    fn get_information_for_uri<S>(self, uri: S) -> Result<Information, Error> where S: Into<String> {
        let uri_string = uri.into();
        let tcp = TcpStream::connect(self.get_host_from_uri(&uri_string)).unwrap();
        let session = self.connect(&uri_string, &tcp)?;

        let command = "ls";

        self.call_ssh_command(command.to_string(), &session);

        Ok(Information::new_for_current_env())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn get_information_for_uri_test() {
        let file_provider = SshProvider {};

        let file_path = PathBuf::from(file!());
        let mut file_path_abs: PathBuf = fs::canonicalize(&file_path).unwrap();
        file_path_abs.pop();
        file_path_abs.pop();
        file_path_abs.pop();

        let mut json_file_path = file_path_abs.clone();
        json_file_path.push("tests/protocol-test-0.1.0.json");

        let information = file_provider.get_information_for_uri(json_file_path.to_str().unwrap()).unwrap();
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
