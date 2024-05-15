use crate::formatter::*;
use crate::printer::*;
use crate::sub_command::SubCommand as FleetSubCommand;
use crate::sub_command::*;
use clap::{App, Arg, SubCommand};

pub mod configuration;
pub mod constants;
pub mod error;
mod filter;
mod formatter;
pub mod information;
mod printer;
mod provider;
mod sub_command;

#[cfg(test)]
mod test_helpers;

fn main() {
    let format_arg = Arg::with_name("format")
        .short("f")
        .long("format")
        .value_name("FORMAT")
        .takes_value(true)
        .help("Defines the output format");
    let configuration_arg = Arg::with_name("config")
        .short("c")
        .long("config")
        .value_name("FILE")
        .help("Sets the configuration file to read")
        .takes_value(true);
    let packages_arg = Arg::with_name("packages")
        .short("p")
        .long("packages")
        .help("Sets if packages are listed");

    let matches = App::new("Fleet Master")
        .version(constants::PROVIDER_VERSION)
        .author("Daniel Corn <info@cundd.net>")
        .about("Gathers information from Fleet providers")
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .subcommand(
            SubCommand::with_name("list")
                .about("Fetch information from all hosts")
                .version(constants::PROVIDER_VERSION)
                .arg(configuration_arg.clone())
                .arg(format_arg.clone())
                .arg(packages_arg.clone()),
        )
        .subcommand(
            SubCommand::with_name("show")
                .about("Fetch information for the given host")
                .version(constants::PROVIDER_VERSION)
                .arg(
                    Arg::with_name("host")
                        .short("h")
                        .long("host")
                        .required(true)
                        .value_name("HOST")
                        .help("Key of the host's configuration")
                        .index(1)
                        .takes_value(true),
                )
                .arg(configuration_arg.clone())
                .arg(format_arg.clone())
                .arg(packages_arg.clone()),
        )
        .subcommand(
            SubCommand::with_name("packages")
                .about("Show packages of hosts")
                .version(constants::PROVIDER_VERSION)
                .arg(
                    Arg::with_name("hosts")
                        .short("h")
                        .long("hosts")
                        .value_name("HOSTS")
                        .help("Comma separated list hosts to fetch the packages")
                        .index(1)
                        .takes_value(true),
                )
                .arg(configuration_arg.clone())
                .arg(format_arg.clone()),
        )
        .subcommand(
            SubCommand::with_name("provide")
                .about("Use the program as information provider")
                .version(constants::PROVIDER_VERSION)
                .arg(format_arg.clone())
                .arg(
                    Arg::with_name("debug")
                        .short("d")
                        .help("print debug information verbosely"),
                ),
        )
        .subcommand(
            SubCommand::with_name("search")
                .about("Search hosts for the given predicate")
                .version(constants::PROVIDER_VERSION)
                .arg(
                    Arg::with_name("package")
                        .short("p")
                        .long("package")
                        .value_name("SEARCH PACKAGE")
                        .help("Package name to search for")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("exact")
                        .long("exact")
                        .help("Only show exact matches"),
                )
                .arg(configuration_arg.clone())
                .arg(format_arg.clone()),
        )
        .subcommand(
            SubCommand::with_name("check")
                .about("Check configurations for all hosts")
                .version(constants::PROVIDER_VERSION)
                .arg(configuration_arg.clone())
                .arg(format_arg.clone()),
        )
        .get_matches();

    let (subcommand, subcommand_matches_option) = get_subcommand(&matches);

    let default_format = match &subcommand {
        &FleetSubCommand::ProvideCommand(_) => "json",
        _ => "console",
    };

    let formatter = get_formatter(default_format, subcommand_matches_option).unwrap();

    Printer::print_if_error(subcommand.exec(&formatter, subcommand_matches_option));
}
