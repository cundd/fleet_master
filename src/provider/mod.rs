mod file_provider;
mod ssh_provider;

use information::*;

pub trait Provider {
    fn get_information_for_uri<S>(self, uri: S) -> Result<Information, Error> where S: Into<String>;
}
