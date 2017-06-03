use clap::ArgMatches;

use error::Error;
use formatter::FormatterTrait;
use printer::Printer;
use sub_command::SshCommandTrait;
use sub_command::SubCommandTrait;

pub struct PackagesCommand;

impl SshCommandTrait for PackagesCommand {}

impl SubCommandTrait for PackagesCommand {
    fn exec<F: FormatterTrait>(&self, formatter: &F, subcommand_matches_option: Option<&ArgMatches>) -> Result<(), Error> {
        let hosts_result = self.get_hosts(subcommand_matches_option.unwrap());

        let (information_collection, error_collection) = match hosts_result {
            Some(hosts) => {
                if hosts.len() == 0 {
                    return Err(Error::new("No hosts given"));
                }
                self.fetch_information_for_hosts(hosts, subcommand_matches_option)?
            },
            None => self.fetch_information_collection(subcommand_matches_option)?,
        };

        Printer::print_error_collection(error_collection);
        Printer::print_result(formatter.format_packages_from_information_collection(information_collection));

        Ok(())
    }
}
