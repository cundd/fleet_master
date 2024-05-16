mod console_formatter;
mod json_formatter;

pub use self::console_formatter::ConsoleFormatter;
pub use self::json_formatter::JsonFormatter;
use crate::error::*;
use crate::information::*;
use crate::shell::ShellOutputCollection;

type FormatterResult = Result<String, Error>;

/// Trait for formatter implementations
pub trait FormatterTrait {
    /// Format the given [`Information`]
    ///
    /// Some implementations may ignore `show_packages`
    fn format_information(
        &self,
        host: &str,
        information: &Information,
        show_packages: bool,
    ) -> FormatterResult;

    /// Format all [`Information`] objects in the collection
    ///
    /// Some implementations may ignore `show_packages`
    fn format_information_collection(
        &self,
        information: InformationCollection,
        show_packages: bool,
    ) -> FormatterResult;

    /// Format the given [`Packages`]
    fn format_packages(&self, packages: &Packages) -> FormatterResult;

    /// Format the [`Packages`] of all [`Information`] objects in the collection
    fn format_packages_from_information_collection(
        &self,
        information_collection: InformationCollection,
    ) -> FormatterResult;

    /// Format all the output from shell execution in the collection
    fn format_shell_output_collection(&self, collection: ShellOutputCollection) -> FormatterResult;
}

/// Wrapper around the different formatter types
pub enum Formatter {
    Json(JsonFormatter),
    Console(ConsoleFormatter),
}

impl FormatterTrait for Formatter {
    fn format_information(
        &self,
        host: &str,
        information: &Information,
        show_packages: bool,
    ) -> FormatterResult {
        match self {
            Formatter::Json(ref f) => f.format_information(host, information, show_packages),
            Formatter::Console(ref f) => f.format_information(host, information, show_packages),
        }
    }

    fn format_information_collection(
        &self,
        information: InformationCollection,
        show_packages: bool,
    ) -> FormatterResult {
        match self {
            Formatter::Json(ref f) => f.format_information_collection(information, show_packages),
            Formatter::Console(ref f) => {
                f.format_information_collection(information, show_packages)
            }
        }
    }

    fn format_packages(&self, packages: &Packages) -> FormatterResult {
        match self {
            Formatter::Json(ref f) => f.format_packages(packages),
            Formatter::Console(ref f) => f.format_packages(packages),
        }
    }

    fn format_packages_from_information_collection(
        &self,
        information_collection: InformationCollection,
    ) -> FormatterResult {
        match self {
            Formatter::Json(ref f) => {
                f.format_packages_from_information_collection(information_collection)
            }
            Formatter::Console(ref f) => {
                f.format_packages_from_information_collection(information_collection)
            }
        }
    }

    fn format_shell_output_collection(&self, collection: ShellOutputCollection) -> FormatterResult {
        match self {
            Formatter::Json(ref f) => f.format_shell_output_collection(collection),
            Formatter::Console(ref f) => f.format_shell_output_collection(collection),
        }
    }
}

/// Returns the formatter for the given format string
pub fn get_formatter(format: &str) -> Result<Formatter, Error> {
    match format {
        "json" => Ok(Formatter::Json(JsonFormatter {})),
        "console" => Ok(Formatter::Console(ConsoleFormatter {})),
        _ => Err(Error::new(format!(
            "No formatter found for format {}",
            format
        ))),
    }
}
