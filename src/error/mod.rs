#[allow(clippy::module_inception)]
mod error;
mod flat_error;

use std::collections::HashMap;

pub use self::flat_error::FlatError as Error;

pub type ErrorCollection = HashMap<String, Error>;
