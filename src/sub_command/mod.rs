mod list_command;
mod show_command;
mod packages_command;
mod provide_command;
mod search_command;

use std::path::PathBuf;
use clap::ArgMatches;

use error::Error;
use error::ErrorCollection;
use formatter::*;
use information::*;
use self::list_command::ListCommand;
use self::show_command::ShowCommand;
use self::packages_command::PackagesCommand;
use self::provide_command::ProvideCommand;
use self::search_command::SearchCommand;
use configuration::*;
use provider::*;

/// Trait for subcommands
pub trait SubCommandTrait {
    /// Performs the subcommand's task(s)
    fn exec<F: FormatterTrait>(&self, formatter: &F, matches_option: Option<&ArgMatches>) -> Result<(), Error>;

    /// Returns the path to the configuration file
    ///
    /// If the `--config` argument is given it's value will be returned, otherwise
    /// [`detect_configuration_file`] is used to search for a configuration file in the current
    /// working directory
    fn get_configuration_file(&self, matches_option: Option<&ArgMatches>) -> Result<PathBuf, Error> {
        if let Some(matches) = matches_option {
            match matches.value_of("config") {
                Some(c) => Ok(PathBuf::from(c)),
                None => detect_configuration_file()
            }
        } else {
            Err(Error::new("Could not detect the configuration file: No arguments are passed to the subcommand"))
        }
    }

    /// Returns the host specified by the `host` argument
    fn get_host<'a>(&self, matches_option: &'a ArgMatches) -> Option<&'a str> {
        matches_option.value_of("host")
    }

    /// Returns the hosts specified by the `hosts` argument
    fn get_hosts<'a>(&self, matches_option: &'a ArgMatches) -> Option<Vec<&'a str>> {
        if let Some(hosts_input) = matches_option.value_of("hosts") {
            let hosts = hosts_input
                .split(",")
                .map(|host| host.trim())
                .filter(|host| host.len() > 0)
                .collect::<Vec<&str>>();

            return Some(hosts);
        }

        None
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
    fn fetch_information_for_requested_host(&self, matches_option: Option<&ArgMatches>) -> Result<(String, Information), Error> {
        let host = match matches_option.unwrap().value_of("host") {
            Some(host) => host,
            None => return Err(Error::new("Argument 'host' not specified")),
        };

        match self.fetch_information_for_host(host, matches_option) {
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
    fn fetch_information_for_requested_hosts(&self, matches_option: Option<&ArgMatches>) -> CollectionResult {
        let hosts = match self.get_hosts(matches_option.unwrap()) {
            Some(hosts) => hosts,
            None => return Err(Error::new("Argument 'hosts' not specified")),
        };

        self.fetch_information_for_hosts(hosts, matches_option)
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
        let filtered: ConfigurationCollection = configuration_collection.into_iter().filter(
            |&(ref host, _)| host.len() > 0 && hosts.contains(&host.as_str())
        ).collect();

        if filtered.len() == 0 {
            Err(Error::new(format!(
                "{}: {}",
                if hosts.len() > 1 { "No configurations found for hosts" } else { "No configuration found for host" },
                hosts.join(", ")
            )))
        } else {
            Ok(self.fetch_information_for_configuration_collection(filtered))
        }
    }

    /// Fetch the information for all hosts in the configuration collection
    fn fetch_information_collection(&self, matches_option: Option<&ArgMatches>) -> CollectionResult {
        let configuration_file = self.get_configuration_file(matches_option)?;
        let configuration_collection = match ConfigurationProvider::load(configuration_file.as_path()) {
            Ok(c) => c,
            Err(e) => return Err(Error::new(
                format!(
                    "Error when loading configuration file '{}': {}",
                    configuration_file.to_string_lossy(),
                    e.to_string()
                ),
            ))
        };

        Ok(self.fetch_information_for_configuration_collection(configuration_collection))
    }

    /// Fetch the information for all hosts in the given configuration collection
    fn fetch_information_for_configuration_collection(&self, configuration_collection: ConfigurationCollection) -> (InformationCollection, ErrorCollection) {
        SshProvider::new().get_information_for_collection(configuration_collection)
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
    PackagesCommand(PackagesCommand),
    ProvideCommand(ProvideCommand),
    SearchCommand(SearchCommand),
}

impl SubCommandTrait for SubCommand {
    fn exec<F: FormatterTrait>(&self, formatter: &F, matches_option: Option<&ArgMatches>) -> Result<(), Error> {
        match self {
            &SubCommand::ListCommand(ref c) => c.exec(formatter, matches_option),
            &SubCommand::ShowCommand(ref c) => c.exec(formatter, matches_option),
            &SubCommand::PackagesCommand(ref c) => c.exec(formatter, matches_option),
            &SubCommand::ProvideCommand(ref c) => c.exec(formatter, matches_option),
            &SubCommand::SearchCommand(ref c) => c.exec(formatter, matches_option),
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
    if let Some(subcommand_matches) = matches.subcommand_matches("packages") {
        return (SubCommand::PackagesCommand(PackagesCommand {}), Some(subcommand_matches));
    }
    if let Some(subcommand_matches) = matches.subcommand_matches("search") {
        return (SubCommand::SearchCommand(SearchCommand {}), Some(subcommand_matches));
    }

    // Default to `provide`
    (SubCommand::ProvideCommand(ProvideCommand {}), None)
}

