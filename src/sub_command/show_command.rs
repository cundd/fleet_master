use clap::ArgMatches;
use std::path::PathBuf;
use configuration::*;
use error::Error;
use formatter::Formatter;
use information::*;
use printer::Printer;
use provider::*;
use sub_command::SubCommand;

pub struct ShowCommand;

impl ShowCommand {
    fn fetch_information(&self, configuration: &Configuration) -> Result<Information, Error> {
        SshProvider::new().get_information(configuration)
    }
}

impl SubCommand for ShowCommand {
    fn exec(&self, formatter: &Formatter, matches: &ArgMatches) -> Result<(), Error> {
        let configuration_file = matches.value_of("config").unwrap();
        let host = matches.value_of("host").unwrap();
        let configuration_collection = ConfigurationProvider::load(PathBuf::from(configuration_file).as_path())?;

        match configuration_collection.get(host) {
            Some(configuration) => Printer::print_result(
                formatter.format_information(self.fetch_information(&configuration))
            ),
            None => Printer::print_error(
                Error::new(format!("Host {} not found in configuration file {}", host, configuration_file))
            ),
        };

        Ok(())
    }
}
