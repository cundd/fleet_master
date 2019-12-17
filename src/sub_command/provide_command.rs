use clap::ArgMatches;
use crate::error::Error;
use crate::information::Platform;
use crate::formatter::*;
use crate::printer::Printer;
use crate::provider::*;
use crate::sub_command::SubCommandTrait;

pub struct ProvideCommand;

impl SubCommandTrait for ProvideCommand {
    fn exec<F: FormatterTrait>(&self, formatter: &F, subcommand_matches_option: Option<&ArgMatches>) -> Result<(), Error> {
        let no_packages = match subcommand_matches_option {
            Some(matches) => matches.is_present("no-packages"),
            None => false
        };

        let platform = Platform::new_for_current_env();
        let result = LocalProvider::new().get_information();

        Printer::print_result(
            match result {
                Ok(information) => formatter.format_information(&platform.host, &information, !no_packages),
                Err(e) => Err(e),
            }
        );

        Ok(())
    }
}
