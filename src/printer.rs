use ansi_term::Colour;
use error::Error;
use error::ErrorCollection;

pub struct Printer;

impl Printer {
    /// Prints the result
    pub fn print_result(o: Result<String, Error>) {
        match o {
            Ok(text) => { println!("{}", text); }
            Err(e) => Self::print_error(e),
        }
    }

    /// Prints an error's message to STDERR painted in red
    pub fn print_error(e: Error) {
        eprintln!("{}", Colour::Red.paint(format!("error: {}", e.message())));
    }

    /// Prints an error if the result is not ok. The Ok part of the result is ignored
    pub fn print_if_error<A>(e: Result<A, Error>) {
        if let Err(e) = e {
            Self::print_error(e);
        }
    }

    /// Prints all errors in the given collection
    #[allow(unused)]
    pub fn print_error_collection(collection: ErrorCollection) {
        for (host, e) in collection {
            eprintln!(
                "{}",
                Colour::Red.paint(format!(
                    "Error for host \"{}\": {}",
                    host,
                    prepare_message(e.message())
                ))
            );
        }
    }

    /// Prints a custom message and the error's message to STDERR painted in red
    #[allow(unused)]
    pub fn print_message_and_error(message: &str, e: Error) {
        eprintln!(
            "{}",
            Colour::Red.paint(format!(
                "error: {}: {}",
                message,
                prepare_message(e.message())
            ))
        );
    }
}

pub fn prepare_message(message: &str) -> String {
    let single_line = message.replace("\n", " ");
    if single_line.chars().count() > 120 {
        format!("{}…", single_line.chars().take(120).collect::<String>())
    } else {
        single_line
    }
}
