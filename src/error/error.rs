use std::error;
use std::fmt;

#[derive(Debug)]
pub struct Error {
    msg: String,
    cause: Option<&'static dyn error::Error>,
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.msg
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        self.cause
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.msg)
    }
}
