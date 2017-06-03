use clap::ArgMatches;
use error::Error;
use formatter::*;
use printer::Printer;
use sub_command::SshCommandTrait;
use sub_command::SubCommandTrait;

pub struct PackagesCommand;

impl SshCommandTrait for PackagesCommand {}

impl SubCommandTrait for PackagesCommand {
    fn exec<F: FormatterTrait>(&self, formatter: &F, subcommand_matches_option: Option<&ArgMatches>) -> Result<(), Error> {
        let (information_collection, _) = match self.get_hosts(subcommand_matches_option.unwrap()) {
            Some(_) => self.fetch_information_for_requested_hosts(subcommand_matches_option)?,
            None => self.fetch_information_collection(subcommand_matches_option)?,
        };
        Printer::print_result(formatter.format_packages_from_information_collection(information_collection));

        Ok(())
    }
}
