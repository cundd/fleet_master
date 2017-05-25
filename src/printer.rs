use std::io::stderr;
use std::io::stdout;
use std::io::Write;
use ansi_term::Colour;
use error::Error;

pub struct Printer;

impl Printer {
    /// Prints the result
    pub fn print_result(o: Result<String, Error>) {
        match o {
            Ok(text) => { let _ = writeln!(&mut stdout(), "{}", text); }
            Err(e) => Self::print_error(e),
        }
    }

    /// Prints an error's message to STDERR painted in red
    pub fn print_error(e: Error) {
        let _ = writeln!(&mut stderr(), "{}", Colour::Red.paint(format!("error: {}", e.message())));
    }

    /// Prints an error if the result is not ok. The Ok part of the result is ignored
    pub fn print_if_error<A>(e: Result<A, Error>) {
        if let Err(e) = e {
            Self::print_error(e);
        }
    }

    /// Prints a custom message and the error's message to STDERR painted in red
    pub fn print_message_and_error(message: &str, e: Error) {
        let _ = writeln!(&mut stderr(), "{}", Colour::Red.paint(format!("error: {}: {}", message, e.message())));
    }
}
