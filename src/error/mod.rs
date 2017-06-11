#[allow(unused)]
mod error;
mod flat_error;

use std::error::Error as StdError;
use std::marker::Sized;
use std::fmt::Display;
use std::collections::HashMap;

pub use self::flat_error::FlatError as Error;

pub type ErrorCollection = HashMap<String, Error>;

trait FleetError: StdError + Display {
    fn new<S: Into<String>>(message: S) -> Self;

    fn from_error<E>(error: E) -> Self where E: 'static + StdError + Sized;

    fn with_error_and_details<E, S: Into<String>>(error: E, message: S) -> Self where E: 'static + StdError + Sized;

    fn message(&self) -> &str;
}
