use std::io::stderr;
use std::io::Write;
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
        let _ = writeln!(&mut stderr(), "ERR: {}", e.message());
    }

    pub fn print_error_if_not_ok<A>(e: Result<A, Error>) {
        if let Err(e) = e {
            Self::print_error(e);
        }
    }

    pub fn print_formatted_error(format: &str, e: Error) {
        let _ = writeln!(&mut stderr(), "ERR: {}: {}", format, e.message());
    }
}
