mod list_command;
mod show_command;
mod show_packages_command;
mod provide_command;

use std::collections::HashMap;
use std::path::PathBuf;
use clap::ArgMatches;

use error::Error;
use formatter::*;
use information::*;
use self::list_command::ListCommand;
use self::show_command::ShowCommand;
use self::show_packages_command::ShowPackagesCommand;
use self::provide_command::ProvideCommand;
use configuration::*;
use provider::*;

type ErrorCollection = HashMap<String, Error>;
type CollectionResult = Result<(InformationCollection, ErrorCollection), Error>;

/// Trait for subcommands
pub trait SubCommandTrait {
    /// Performs the subcommand's task(s)
    fn exec<F: FormatterTrait>(&self, formatter: &F, subcommand_matches_option: Option<&ArgMatches>) -> Result<(), Error>;

    /// Returns the path to the configuration file
    ///
    /// If the `--config` argument is given it's value will be returned, otherwise
    /// [`detect_configuration_file`] is used to search for a configuration file in the current
    /// working directory
    fn get_configuration_file(&self, subcommand_matches_option: Option<&ArgMatches>) -> Result<PathBuf, Error> {
        if let Some(matches) = subcommand_matches_option {
            match matches.value_of("config") {
                Some(c) => Ok(PathBuf::from(c)),
                None => detect_configuration_file()
            }
        } else {
            Err(Error::new("Could not detect the configuration file: No arguments are passed to the subcommand"))
        }
    }

    /// Returns the host specified by the `host` argument
    fn get_host<'a>(&self, subcommand_matches_option: &'a ArgMatches) -> Option<&'a str> {
        subcommand_matches_option.value_of("host")
    }

    /// Returns the hosts specified by the `hosts` argument
    fn get_hosts<'a>(&self, subcommand_matches_option: &'a ArgMatches) -> Option<Vec<&'a str>> {
        match subcommand_matches_option.value_of("hosts") {
            Some(hosts) => Some(hosts.split(",").collect::<Vec<&str>>()),
            None => None
        }
    }
}

/// Trait for SSH-based subcommands
pub trait SshCommandTrait: SubCommandTrait {
    /// Fetch information for the host specified in the `--host` argument
    ///
    /// This will perform the following steps:
    ///     1. Look for the configuration file
    ///     2. Get the host from the arguments
    ///     3. Fetch the configuration for the host
    ///     4. Fetch the information using the configuration
    fn fetch_information_for_requested_host(&self, subcommand_matches_option: Option<&ArgMatches>) -> Result<(String, Information), Error> {
        let host = match subcommand_matches_option.unwrap().value_of("host") {
            Some(host) => host,
            None => return Err(Error::new("Argument 'host' not specified")),
        };

        match self.fetch_information_for_host(host, subcommand_matches_option) {
            Ok(i) => Ok((host.to_owned(), i)),
            Err(e) => Err(e),
        }
    }

    /// Fetch information for the hosts specified in the `--hosts` argument
    ///
    /// This will perform the following steps:
    ///     1. Look for the configuration file
    ///     2. Get the host from the arguments
    ///     3. Fetch the configuration for the host
    ///     4. Fetch the information using the configuration
    fn fetch_information_for_requested_hosts(&self, subcommand_matches_option: Option<&ArgMatches>) -> CollectionResult {
        let hosts = match self.get_hosts(subcommand_matches_option.unwrap()) {
            Some(hosts) => hosts,
            None => return Err(Error::new("Argument 'hosts' not specified")),
        };

        self.fetch_information_for_hosts(hosts, subcommand_matches_option)
    }

    /// Fetch information for the given host
    ///
    /// This will perform the following steps:
    ///     1. Look for the configuration file
    ///     2. Fetch the configuration for the host
    ///     3. Fetch the information using the configuration
    fn fetch_information_for_host(&self, host: &str, matches_option: Option<&ArgMatches>) -> Result<Information, Error> {
        let configuration_file = self.get_configuration_file(matches_option)?;
        let configuration = ConfigurationProvider::get_configuration_for_host(configuration_file.as_path(), host)?;

        self.fetch_information(&configuration)
    }

    /// Fetch information for the given hosts
    ///
    /// This will perform the following steps:
    ///     1. Look for the configuration file
    ///     2. Filter the list of configurations
    ///     3. Fetch the information using the configurations
    fn fetch_information_for_hosts(&self, hosts: Vec<&str>, matches_option: Option<&ArgMatches>) -> CollectionResult {
        let configuration_file = self.get_configuration_file(matches_option)?;
        let configuration_collection = ConfigurationProvider::load(configuration_file.as_path())?;
        let filtered: ConfigurationCollection = configuration_collection.into_iter().filter(|&(ref host, _)| (hosts.contains(&host.as_str()))).collect();

        if filtered.len() == 0 {
            if hosts.len() > 1 {
                return Err(Error::new(format!("No configuration found for host: {}", hosts.join(", "))));
            }
            return Err(Error::new(format!("No configurations found for hosts: {}", hosts.join(", "))));
        }

        Ok(self.fetch_information_for_configuration_collection(filtered))
    }

    /// Fetch the information for all hosts in the configuration collection
    fn fetch_information_collection(&self, subcommand_matches_option: Option<&ArgMatches>) -> CollectionResult {
        let configuration_file = self.get_configuration_file(subcommand_matches_option)?;
        let configuration_collection = ConfigurationProvider::load(configuration_file.as_path())?;

        Ok(self.fetch_information_for_configuration_collection(configuration_collection))
    }

    /// Fetch the information for all hosts in the given configuration collection
    fn fetch_information_for_configuration_collection(&self, configuration_collection: ConfigurationCollection) -> (InformationCollection, ErrorCollection) {
        let mut error_collection = ErrorCollection::new();
        let mut information_collection = InformationCollection::new();

        for (host, configuration) in configuration_collection {
            match self.fetch_information(&configuration) {
                Ok(i) => { let _ = information_collection.insert(host, i); }
                Err(e) => { let _ = error_collection.insert(host, e); }
            };
        }

        (information_collection, error_collection)
    }

    /// Fetch information from the host in the given configuration
    fn fetch_information(&self, configuration: &Configuration) -> Result<Information, Error> {
        SshProvider::new().get_information(configuration)
    }
}

/// Wrapper type to dispatch to the concrete subcommand
pub enum SubCommand {
    ListCommand(ListCommand),
    ShowCommand(ShowCommand),
    ShowPackagesCommand(ShowPackagesCommand),
    ProvideCommand(ProvideCommand),
}

impl SubCommandTrait for SubCommand {
    fn exec<F: FormatterTrait>(&self, formatter: &F, subcommand_matches_option: Option<&ArgMatches>) -> Result<(), Error> {
        match self {
            &SubCommand::ListCommand(ref c) => c.exec(formatter, subcommand_matches_option),
            &SubCommand::ShowCommand(ref c) => c.exec(formatter, subcommand_matches_option),
            &SubCommand::ShowPackagesCommand(ref c) => c.exec(formatter, subcommand_matches_option),
            &SubCommand::ProvideCommand(ref c) => c.exec(formatter, subcommand_matches_option),
        }
    }
}

pub fn get_subcommand<'x>(matches: &'x ArgMatches) -> (SubCommand, Option<&'x ArgMatches<'x>>) {
    if let Some(subcommand_matches) = matches.subcommand_matches("list") {
        return (SubCommand::ListCommand(ListCommand {}), Some(subcommand_matches));
    }
    if let Some(subcommand_matches) = matches.subcommand_matches("show") {
        return (SubCommand::ShowCommand(ShowCommand {}), Some(subcommand_matches));
    }
    if let Some(subcommand_matches) = matches.subcommand_matches("show-packages") {
        return (SubCommand::ShowPackagesCommand(ShowPackagesCommand {}), Some(subcommand_matches));
    }

    // Default to provide
    (SubCommand::ProvideCommand(ProvideCommand {}), None)
}

