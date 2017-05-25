use clap::ArgMatches;
use error::Error;
use formatter::*;
use printer::Printer;
use sub_command::SshCommandTrait;
use sub_command::SubCommandTrait;

pub struct ShowPackagesCommand;

impl SshCommandTrait for ShowPackagesCommand {}

impl SubCommandTrait for ShowPackagesCommand {
    fn exec<F: FormatterTrait>(&self, formatter: &F, subcommand_matches_option: Option<&ArgMatches>) -> Result<(), Error> {
        let (_, information) = self.fetch_information_for_requested_host(subcommand_matches_option)?;
        Printer::print_result(formatter.format_packages(information));

        Ok(())
    }
}
