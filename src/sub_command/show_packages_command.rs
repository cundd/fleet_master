use clap::ArgMatches;
use configuration::*;
use error::Error;
use formatter::*;
use information::*;
use printer::Printer;
use provider::*;
use sub_command::SubCommandTrait;

pub struct ShowPackagesCommand;

impl ShowPackagesCommand {
    fn fetch_information(&self, configuration: &Configuration) -> Result<Information, Error> {
        SshProvider::new().get_information(configuration)
    }
}

impl SubCommandTrait for ShowPackagesCommand {
    fn exec<F: FormatterTrait>(&self, formatter: &F, subcommand_matches_option: Option<&ArgMatches>) -> Result<(), Error> {
//        let matches = subcommand_matches_option.unwrap();
//        let configuration_file = self.get_configuration_file(subcommand_matches_option)?;
//
//        let host = matches.value_of("host").unwrap();
//        let configuration_collection = ConfigurationProvider::load(configuration_file.as_path())?;
//        let show_packages = matches.is_present("packages");
//
//        match configuration_collection.get(host) {
//            Some(configuration) => {
//                let information = self.fetch_information(&configuration);
//
//                Printer::print_result(formatter.format_information(&host, information, show_packages))
//            }
//            None => Printer::print_error(
//                Error::new(format!("Host {} not found in configuration file {}", host, configuration_file.to_str().unwrap()))
//            ),
//        };

        println!("Show Packages here");
        Ok(())
    }
}
