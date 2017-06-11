use std::error;
use std::fmt;
use std::marker::Sized;

#[derive(Debug)]
pub struct FlatError {
    msg: String,
}

impl FlatError {
    pub fn new<S: Into<String>>(message: S) -> Self {
        Self {
            msg: message.into(),
        }
    }
    pub fn from_error<E>(error: E) -> Self where E: 'static + error::Error + Sized {
        Self {
            msg: error.description().to_owned(),
        }
    }

    pub fn with_error_and_details<E, S: Into<String>>(error: E, message: S) -> Self where E: 'static + error::Error + Sized {
        Self {
            msg: error.description().to_owned() + " (Details: '" + &message.into() + "')",
        }
    }

    pub fn message(&self) -> &str {
        &self.msg
    }
}

impl super::FleetError for FlatError {
    fn new<S: Into<String>>(message: S) -> Self {
        Self::new(message)
    }

    fn from_error<E>(error: E) -> Self where E: 'static + error::Error + Sized {
        Self::from_error(error)
    }

    fn with_error_and_details<E, S: Into<String>>(error: E, message: S) -> Self where E: 'static + error::Error + Sized {
        Self::with_error_and_details(error, message)
    }

    fn message(&self) -> &str {
        Self::message(&self)
    }
}

impl error::Error for FlatError {
    fn description(&self) -> &str {
        self.message()
    }
}

impl fmt::Display for FlatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}
