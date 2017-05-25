mod json_formatter;
mod console_formatter;

pub use self::console_formatter::ConsoleFormatter;
pub use self::json_formatter::JsonFormatter;

use clap::ArgMatches;

use error::Error;
use information::*;


/// Trait for formatter implementations
pub trait FormatterTrait {
    fn format_information(&self, host: &str, information: Result<Information, Error>, show_packages: bool) -> Result<String, Error>;
    fn format_information_collection(&self, information: InformationCollection, show_packages: bool) -> Result<String, Error>;
}


/// Wrapper around the different formatter types
pub enum Formatter {
    Json(JsonFormatter),
    Console(ConsoleFormatter),
}

impl FormatterTrait for Formatter {
    fn format_information(&self, host: &str, information: Result<Information, Error>, show_packages: bool) -> Result<String, Error> {
        match self {
            &Formatter::Json(ref f) => f.format_information(host, information, show_packages),
            &Formatter::Console(ref f) => f.format_information(host, information, show_packages),
        }
    }

    fn format_information_collection(&self, information: InformationCollection, show_packages: bool) -> Result<String, Error> {
        match self {
            &Formatter::Json(ref f) => f.format_information_collection(information, show_packages),
            &Formatter::Console(ref f) => f.format_information_collection(information, show_packages),
        }
    }
}

/// Returns the formatter for the matches
pub fn get_formatter<'x>(default_format: &str, matches_option: Option<&'x ArgMatches<'x>>) -> Result<Formatter, Error> {
    get_formatter_for_format(&get_format(default_format, matches_option))
}

/// Returns the formatter for the given format string
fn get_formatter_for_format(format: &str) -> Result<Formatter, Error> {
    match format {
        "json" => Ok(Formatter::Json(JsonFormatter {})),
        "console" => Ok(Formatter::Console(ConsoleFormatter {})),
        _ => Err(Error::new(format!("No formatter found for format {}", format)))
    }
}

fn get_format<'x>(default_format: &'x str, matches_option: Option<&'x ArgMatches<'x>>) -> &'x str {
    match matches_option {
        Some(matches) => matches.value_of("format").unwrap_or(default_format),
        None => default_format,
    }
}
