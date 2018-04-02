use std::error::Error as StdError;
use std::fmt;
use ssh2::Error as Ssh2Error;

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
    pub fn from_error(error: &StdError) -> Self {
        Self {
            msg: error.description().to_owned(),
        }
    }

    pub fn with_error_and_details<S: Into<String>>(error: &StdError, message: S) -> Self {
        Self {
            msg: format!("{} (Details: '{}')", error.description(), message.into()),
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

    fn from_error(error: &StdError) -> Self {
        Self::from_error(error)
    }

    fn with_error_and_details<S: Into<String>>(error: &StdError, message: S) -> Self {
        Self::with_error_and_details(error, message)
    }

    fn message(&self) -> &str {
        Self::message(&self)
    }
}

impl StdError for FlatError {
    fn description(&self) -> &str {
        self.message()
    }
}

impl fmt::Display for FlatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl<'a> From<&'a StdError> for FlatError {
    fn from(error: &StdError) -> Self {
        FlatError::from_error(error)
    }
}

impl From<Ssh2Error> for FlatError {
    fn from(error: Ssh2Error) -> Self {
        FlatError::from_error(&error)
    }
}

impl<'a> From<&'a Ssh2Error> for FlatError {
    fn from(error: &Ssh2Error) -> Self {
        FlatError::from_error(error)
    }
}
