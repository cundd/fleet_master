mod command;
mod configuration;
mod constants;
mod error;
mod filter;
mod formatter;
mod information;
mod printer;
mod provider;
mod shell;

#[cfg(test)]
mod test_helpers;

use crate::command::*;
use crate::configuration::detect_configuration_file;
use crate::formatter::*;
use crate::printer::*;
use clap::{Parser, Subcommand};
use error::Error;
use std::io::IsTerminal;
use std::path::PathBuf;

/// Fleet Master
#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// Set the configuration file to read
    #[arg(short, long, value_parser=clap::value_parser!(PathBuf))]
    pub config: Option<PathBuf>,

    /// The output format
    #[arg(value_enum, default_value_t)]
    pub color: ColorSupport,

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

    /// Execute the hosts update command
    Update(UpdateArgs),

    /// Execute a shell command on the hosts
    Exec(ExecArgs),
}

#[derive(Clone, Copy, Default, Debug, clap::ValueEnum)]
enum ColorSupport {
    #[default]
    Auto,
    Always,
    No,
}

impl From<String> for ColorSupport {
    fn from(value: String) -> Self {
        match value.as_str() {
            "always" => ColorSupport::Always,
            "no" => ColorSupport::No,
            _ => ColorSupport::default(),
        }
    }
}

fn get_format(cli: &Cli) -> &str {
    let default_format = "console";

    match &cli.command {
        Commands::List(args) => &args.common.format,
        Commands::Show(args) => &args.common.format,
        Commands::Packages(args) => &args.common.format,
        Commands::Search(args) => &args.common.format,
        Commands::Check(args) => &args.common.format,
        Commands::Update(args) => &args.common.format,
        Commands::Exec(args) => &args.common.format,
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
        Commands::Update(args) => &args.common.config,
        Commands::Exec(args) => &args.common.config,
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
    let formatter = get_formatter(
        get_format(&cli),
        match cli.color {
            ColorSupport::Auto => std::io::stdout().is_terminal(),
            ColorSupport::Always => true,
            ColorSupport::No => false,
        },
    )?;

    if let Commands::Provide(_) = cli.command {
        return ProvideCommand::default().provide(&formatter);
    }

    let config_file = get_configuration_file(&cli)?;

    match cli.command {
        Commands::List(args) => ListCommand::default().exec(&formatter, config_file, args),
        Commands::Show(args) => ShowCommand::default().exec(&formatter, config_file, args),
        Commands::Packages(args) => PackagesCommand::default().exec(&formatter, config_file, args),
        Commands::Search(args) => SearchCommand::default().exec(&formatter, config_file, args),
        Commands::Check(args) => CheckCommand::default().exec(&formatter, config_file, args),
        Commands::Update(args) => UpdateCommand::default().exec(&formatter, config_file, args),
        Commands::Exec(args) => ExecCommand::default().exec(&formatter, config_file, args),
        Commands::Provide(_) => unreachable!(),
    }
}
