mod list_command;
mod show_command;
mod show_packages_command;
mod provide_command;

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

/// Trait for subcommands
pub trait SubCommandTrait {
    fn exec<F: FormatterTrait>(&self, formatter: &F, subcommand_matches_option: Option<&ArgMatches>) -> Result<(), Error>;

    fn get_configuration_file(&self, subcommand_matches_option: Option<&ArgMatches>) -> Result<PathBuf, Error> {
        match subcommand_matches_option.unwrap().value_of("config") {
            Some(c) => Ok(PathBuf::from(c)),
            None => detect_configuration_file()
        }
    }
}

/// Trait for SSH-based subcommands
pub trait SshCommandTrait: SubCommandTrait {
    /// Fetch information for the host specified in the --host argument
    ///
    /// This will perform the following steps:
    ///     1. Look for the configuration file
    ///     2. Get the host from the arguments
    ///     3. Fetch the configuration for the host
    ///     4. Fetch the information using the configuration
    fn fetch_information_for_requested_host(&self, subcommand_matches_option: Option<&ArgMatches>) -> Result<(String, Information), Error> {
        let configuration_file = self.get_configuration_file(subcommand_matches_option)?;

        let host = match subcommand_matches_option.unwrap().value_of("host") {
            Some(host) => host,
            None => return Err(Error::new("Argument 'host' not specified")),
        };
        let configuration = ConfigurationProvider::get_configuration_for_host(configuration_file.as_path(), host)?;

        match self.fetch_information(&configuration) {
            Ok(c) => Ok((host.to_owned(), c)),
            Err(e) => Err(e),
        }
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

