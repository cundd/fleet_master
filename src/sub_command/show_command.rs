use clap::ArgMatches;
use crate::error::Error;
use crate::formatter::*;
use crate::printer::Printer;
use crate::sub_command::SshCommandTrait;
use crate::sub_command::SubCommandTrait;

pub struct ShowCommand;

impl SshCommandTrait for ShowCommand {}

impl SubCommandTrait for ShowCommand {
    fn exec<F: FormatterTrait>(&self, formatter: &F, subcommand_matches_option: Option<&ArgMatches>) -> Result<(), Error> {
        let (host, information) = self.fetch_information_for_requested_host(subcommand_matches_option)?;
        let show_packages = subcommand_matches_option.unwrap().is_present("packages");

        Printer::print_result(formatter.format_information(&host, &information, false));
        if show_packages {
            Printer::print_result(formatter.format_packages(&information.packages));
        }

        Ok(())
    }
}
