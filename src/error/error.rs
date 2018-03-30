use std::error;
use std::fmt;
use std::marker::Sized;

use super::FleetError;

#[derive(Debug)]
pub struct Error {
    msg: String,
    cause: Option<&'static error::Error>,
}

impl super::FleetError for Error {
    fn new<S: Into<String>>(message: S) -> Self {
        Self {
            msg: message.into(),
            cause: None,
        }
    }

    fn from_error(error: &error::Error) -> Self {
        Self {
            msg: error.description().to_owned(),
            cause: None, //cause: Some(&error)
        }
    }

    fn with_error_and_details<S: Into<String>>(error: &error::Error, message: S) -> Self {
        Self {
            msg: format!("{} (Details: '{}')", error.description(), message.into()),
            cause: None, //cause: Some(&error)
        }
    }

    fn message(&self) -> &str {
        &self.msg
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        self.message()
    }

    fn cause(&self) -> Option<&error::Error> {
        self.cause
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

