#[allow(clippy::module_inception)]
mod error;
mod flat_error;

use std::collections::HashMap;
use std::error::Error as StdError;
use std::fmt::Display;

pub use self::flat_error::FlatError as Error;

pub type ErrorCollection = HashMap<String, Error>;

trait FleetError: StdError + Display {
    fn new<S: Into<String>>(message: S) -> Self;

    fn from_error(error: &dyn StdError) -> Self;

    fn with_error_and_details<S: Into<String>>(error: &dyn StdError, message: S) -> Self;

    fn message(&self) -> &str;
}
