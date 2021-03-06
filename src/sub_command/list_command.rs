use clap::ArgMatches;

use crate::error::Error;
use crate::formatter::*;
use crate::printer::Printer;
use crate::sub_command::SshCommandTrait;
use crate::sub_command::SubCommandTrait;

pub struct ListCommand;

impl SshCommandTrait for ListCommand {}

impl SubCommandTrait for ListCommand {
    fn exec<F: FormatterTrait>(
        &self,
        formatter: &F,
        subcommand_matches_option: Option<&ArgMatches>,
    ) -> Result<(), Error> {
        let show_packages = match subcommand_matches_option {
            Some(matches) => matches.is_present("packages"),
            None => false,
        };

        let (information_collection, _) =
            match self.fetch_information_collection(subcommand_matches_option) {
                Ok(r) => r,
                Err(e) => return Err(e),
            };

        Printer::print_result(
            formatter.format_information_collection(information_collection, show_packages),
        );

        Ok(())
    }
}
