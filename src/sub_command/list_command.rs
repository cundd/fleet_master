use clap::ArgMatches;
use std::collections::HashMap;
use configuration::*;
use error::Error;
use formatter::*;
use information::*;
use printer::Printer;
use provider::*;
use sub_command::SubCommandTrait;

pub struct ListCommand;

impl ListCommand {
    fn fetch_information(&self, configuration: &Configuration) -> Result<Information, Error> {
        SshProvider::new().get_information(configuration)
    }
}

impl SubCommandTrait for ListCommand {
    fn exec<F: FormatterTrait>(&self, formatter: &F, subcommand_matches_option: Option<&ArgMatches>) -> Result<(), Error> {
        let show_packages = match subcommand_matches_option {
            Some(matches) => matches.is_present("packages"),
            None => false
        };
        let config = self.get_configuration_file(subcommand_matches_option)?;
        let configuration_collection = ConfigurationProvider::load(config.as_path())?;

        let mut information_collection = HashMap::new();
        for (host, configuration) in configuration_collection {
            match self.fetch_information(&configuration) {
                Ok(i) => { let _ = information_collection.insert(host, i); }
                Err(e) => Printer::print_message_and_error(&format!("Error for host entry {}", &host), e),
            };
        }


        Printer::print_result(
            formatter.format_information_collection(information_collection, show_packages)
        );

        Ok(())
    }
}
