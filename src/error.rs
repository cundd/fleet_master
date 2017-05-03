use std;
use std::fmt;
//use std::error::Error as ;

#[derive(Debug)]
pub struct Error {
    msg: String,
    cause: Option<&'static std::error::Error>
}

impl Error {
    pub fn new<S: Into<String>>(message: S) -> Self {
        Error {
            msg: message.into(),
            cause: None
        }
    }

    pub fn from_error<E>(error: E) -> Self where E: 'static + std::error::Error + std::marker::Sized {
        Error {
            msg: error.description().to_owned(),
            cause: None //cause: Some(&error)
        }
    }

    pub fn with_error_and_details<E, S: Into<String>>(error: E, message:S) -> Self where E: 'static + std::error::Error + std::marker::Sized {
        Error {
            msg: error.description().to_owned() + " (Details: '" + &message.into() + "')",
            cause: None //cause: Some(&error)
        }
    }

    pub fn message(&self) -> &str {
        &self.msg
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        self.message()
    }

    fn cause(&self) -> Option<&std::error::Error> {
        self.cause
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}
