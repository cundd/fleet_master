#[allow(unused)]
mod error;
mod flat_error;

use std::error::Error as StdError;
use std::fmt::Display;
use std::collections::HashMap;

pub use self::flat_error::FlatError as Error;

pub type ErrorCollection = HashMap<String, Error>;

trait FleetError: StdError + Display {
    fn new<S: Into<String>>(message: S) -> Self;

    fn from_error(error: &StdError) -> Self;

    fn with_error_and_details<S: Into<String>>(error: &StdError, message: S) -> Self;

    fn message(&self) -> &str;
}
