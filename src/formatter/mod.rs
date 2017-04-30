mod json_formatter;
mod console_formatter;

pub use self::console_formatter::ConsoleFormatter;
pub use self::json_formatter::JsonFormatter;

use error::Error;
use information::*;


/// Trait for formatter implementations
pub trait FormatterTrait {
    fn format_information(&self, host: &str, information: Result<Information, Error>) -> Result<String, Error>;
    fn format_information_collection(&self, information: InformationCollection) -> Result<String, Error>;
}


/// Wrapper around the different formatter types
pub enum Formatter {
    Json(JsonFormatter),
    Console(ConsoleFormatter),
}

impl FormatterTrait for Formatter {
    fn format_information(&self, host: &str, information: Result<Information, Error>) -> Result<String, Error> {
        match self {
            &Formatter::Json(ref f) => f.format_information(host, information),
            &Formatter::Console(ref f) => f.format_information(host, information),
        }
    }

    fn format_information_collection(&self, information: InformationCollection) -> Result<String, Error> {
        match self {
            &Formatter::Json(ref f) => f.format_information_collection(information),
            &Formatter::Console(ref f) => f.format_information_collection(information),
        }
    }
}


/// Returns the formatter for the given format string
pub fn get_formatter(format: &str) -> Result<Formatter, Error> {
    match format {
        "json" => Ok(Formatter::Json(JsonFormatter {})),
        "console" => Ok(Formatter::Console(ConsoleFormatter {})),
        _ => Err(Error::new(format!("No formatter found for format {}", format)))
    }
}
