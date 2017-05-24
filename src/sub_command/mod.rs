mod list_command;
mod show_command;
mod provide_command;

use std::path::PathBuf;
use clap::ArgMatches;
use error::Error;
use formatter::*;
use self::list_command::ListCommand;
use self::show_command::ShowCommand;
use self::provide_command::ProvideCommand;
use configuration::*;

pub trait SubCommandTrait {
    fn exec<F: FormatterTrait>(&self, formatter: &F, subcommand_matches_option: Option<&ArgMatches>) -> Result<(), Error>;

    fn get_configuration_file(&self, subcommand_matches_option: Option<&ArgMatches>) -> Result<PathBuf, Error> {
        match subcommand_matches_option.unwrap().value_of("config") {
            Some(c) => Ok(PathBuf::from(c)),
            None => detect_configuration_file()
        }
    }
}


pub enum SubCommand {
    ListCommand(ListCommand),
    ShowCommand(ShowCommand),
    ProvideCommand(ProvideCommand),
}

impl SubCommandTrait for SubCommand {
    fn exec<F: FormatterTrait>(&self, formatter: &F, subcommand_matches_option: Option<&ArgMatches>) -> Result<(), Error> {
        match self {
            &SubCommand::ListCommand(ref c) => c.exec(formatter, subcommand_matches_option),
            &SubCommand::ShowCommand(ref c) => c.exec(formatter, subcommand_matches_option),
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

    // Default to provide
    (SubCommand::ProvideCommand(ProvideCommand {}), None)
}

