use std::path::PathBuf;

use dirs;

pub struct Helper;

impl Helper {
    pub fn get_ssh_dir() -> PathBuf {
        let mut path = dirs::home_dir().unwrap();
        path.push(".ssh");

        path
    }

    pub fn get_ssh_file_path<'a, S: Into<&'a str>>(file: S) -> PathBuf {
        let mut path = Helper::get_ssh_dir();
        path.push(file.into());

        path
    }
}
