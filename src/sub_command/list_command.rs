use clap::ArgMatches;
use std::path::PathBuf;
use configuration::*;
use error::Error;
use formatter::Formatter;
use information::*;
use printer::Printer;
use provider::*;
use sub_command::SubCommand;

pub struct ListCommand;

impl ListCommand {
    fn fetch_information(&self, configuration: &Configuration) -> Result<Information, Error> {
        SshProvider::new().get_information(configuration)
    }
}

impl SubCommand for ListCommand {
    fn exec(&self, formatter: &Formatter, matches: &ArgMatches) -> Result<(), Error> {
        let config = matches.value_of("config").unwrap();

        let configuration_collection = ConfigurationProvider::load(PathBuf::from(config).as_path())?;

        let mut information_collection = vec![];
        for (configuration_entry, configuration) in configuration_collection {
            match self.fetch_information(&configuration) {
                Ok(i) => information_collection.push(i),
                Err(e) => Printer::print_formatted_error(&format!("Error for host entry {}", &configuration_entry), e),
            }
        }


        Printer::print_result(
            formatter.format_information_collection(information_collection)
        );

        Ok(())
    }
}
