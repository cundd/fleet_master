use std::io::stderr;
use std::io::Write;
use ansi_term::Colour;
use error::Error;

pub struct Printer;

impl Printer {
    pub fn print_result(o: Result<String, Error>) {
        match o {
            Ok(text) => println!("{}", text),
            Err(e) => Self::print_error(e),
        }
    }

    pub fn print_error(e: Error) {
        let _ = writeln!(&mut stderr(), "{}", Colour::Red.paint(format!("error: {}", e.message())));
    }

    pub fn print_error_if_not_ok<A>(e: Result<A, Error>) {
        if let Err(e) = e {
            Self::print_error(e);
        }
    }

    pub fn print_message_and_error(message: &str, e: Error) {
        let _ = writeln!(&mut stderr(), "{}", Colour::Red.paint(format!("error: {}: {}", message, e.message())));
    }
}
