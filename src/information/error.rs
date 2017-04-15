use std;

#[derive(Debug)]
pub enum Error {
    Message(String),
}

impl Error {
    pub fn new<S>(message: S) -> Error where S: Into<String> {
        Error::Message(message.into())
    }

    pub fn new_from_error<E>(error: E) -> Error where E: std::error::Error + std::marker::Sized {
        Error::new(format!("{:?}", error))
    }
}
