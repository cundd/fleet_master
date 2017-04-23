mod json_formatter;
mod factory;

use error::Error;
use information::*;

pub use self::json_formatter::JsonFormatter;
pub use self::factory::Factory;

pub trait Formatter {
    fn format_information(&self, information: Result<Information, Error>) -> Result<String, Error>;
    fn format_information_collection(&self, information: InformationCollection) -> Result<String, Error>;
}
