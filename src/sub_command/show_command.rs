use clap::ArgMatches;
use std::path::PathBuf;
use configuration::*;
use error::Error;
use formatter::*;
use information::*;
use printer::Printer;
use provider::*;
use sub_command::SubCommandTrait;

pub struct ShowCommand;

impl ShowCommand {
    fn fetch_information(&self, configuration: &Configuration) -> Result<Information, Error> {
        SshProvider::new().get_information(configuration)
    }
}

impl SubCommandTrait for ShowCommand {
    fn exec<F: FormatterTrait>(&self, formatter: &F, subcommand_matches_option: Option<&ArgMatches>) -> Result<(), Error> {
        let matches = subcommand_matches_option.unwrap();
        let configuration_file = matches.value_of("config").unwrap();

        let host = matches.value_of("host").unwrap();
        let configuration_collection = ConfigurationProvider::load(PathBuf::from(configuration_file).as_path())?;

        match configuration_collection.get(host) {
            Some(configuration) => Printer::print_result(
                formatter.format_information(&host, self.fetch_information(&configuration))
            ),
            None => Printer::print_error(
                Error::new(format!("Host {} not found in configuration file {}", host, configuration_file))
            ),
        };

        Ok(())
    }
}
