extern crate ansi_term;

use ansi_term::Colour;
use clap::ArgMatches;

use crate::error::Error;
use crate::formatter::*;
use crate::printer::prepare_message;
use crate::sub_command::SshCommandTrait;
use crate::sub_command::SubCommandTrait;

pub struct CheckCommand;

impl SshCommandTrait for CheckCommand {}

impl SubCommandTrait for CheckCommand {
    fn exec<F: FormatterTrait>(
        &self,
        _formatter: &F,
        subcommand_matches_option: Option<&ArgMatches>,
    ) -> Result<(), Error> {
        let (information_collection, error_collection) =
            match self.fetch_information_collection(subcommand_matches_option) {
                Ok(r) => r,
                Err(e) => return Err(e),
            };

        for (host, _) in information_collection {
            println!("{}", Colour::Green.paint(format!("[OK] Host \"{}\"", host)));
        }
        for (host, error) in error_collection {
            eprintln!(
                "{}",
                Colour::Red.paint(format!(
                    "[ERROR] Host \"{}\": {}",
                    host,
                    prepare_message(error.message())
                ))
            );
        }

        Ok(())
    }
}
