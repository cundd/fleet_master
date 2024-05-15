mod command;
mod configuration;
mod constants;
mod error;
mod filter;
mod formatter;
mod information;
mod printer;
mod provider;

#[cfg(test)]
mod test_helpers;

use crate::command::*;
use crate::configuration::detect_configuration_file;
use crate::formatter::*;
use crate::printer::*;
use clap::{Parser, Subcommand};
use error::Error;
use std::path::PathBuf;

/// Fleet Master
#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// Set the configuration file to read
    #[arg(short, long, value_parser=clap::value_parser!(PathBuf))]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Fetch information from all hosts
    List(ListArgs),

    /// Fetch information for the given host
    Show(ShowArgs),

    /// Show packages of hosts
    Packages(PackagesArgs),

    /// Use the program as information provider
    Provide(ProvideArgs),

    /// Search hosts for the given predicate
    Search(SearchArgs),

    /// Check configurations for all hosts
    Check(CheckArgs),
}

fn get_format(cli: &Cli) -> &str {
    let default_format = "console";

    match &cli.command {
        Commands::List(args) => &args.common.format,
        Commands::Show(args) => &args.common.format,
        Commands::Packages(args) => &args.common.format,
        Commands::Search(args) => &args.common.format,
        Commands::Check(args) => &args.common.format,
        Commands::Provide(_) => return "json",
    }
    .as_ref()
    .map_or(default_format, |f| f.as_str())
}

fn get_configuration_file(cli: &Cli) -> Result<PathBuf, Error> {
    if let Some(config_file) = &cli.config {
        return Ok(config_file.clone());
    }

    match &cli.command {
        Commands::List(args) => &args.common.config,
        Commands::Show(args) => &args.common.config,
        Commands::Packages(args) => &args.common.config,
        Commands::Search(args) => &args.common.config,
        Commands::Check(args) => &args.common.config,
        Commands::Provide(_) => &None,
    }
    .as_ref()
    .map_or_else(detect_configuration_file, |p| Ok(p.clone()))
}

fn main() {
    Printer::print_if_error(run());
}

fn run() -> Result<(), Error> {
    let cli = Cli::parse();
    let formatter = get_formatter(get_format(&cli))?;

    if let Commands::Provide(_) = cli.command {
        return ProvideCommand::default().provide(&formatter);
    }

    let config_file = get_configuration_file(&cli)?;

    match &cli.command {
        Commands::List(args) => ListCommand::default().exec(&formatter, config_file, args),
        Commands::Show(args) => ShowCommand::default().exec(&formatter, config_file, args),
        Commands::Packages(args) => PackagesCommand::default().exec(&formatter, config_file, args),
        Commands::Search(args) => SearchCommand::default().exec(&formatter, config_file, args),
        Commands::Check(args) => CheckCommand::default().exec(&formatter, config_file, args),
        Commands::Provide(_) => unreachable!(),
    }
}
