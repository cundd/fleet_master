//#![feature(slice_concat_ext)]
//#![feature(test)]
//extern crate test;


extern crate serde;
extern crate serde_json;
extern crate serde_yaml;
extern crate ssh2;
extern crate clap;
extern crate ansi_term;

#[macro_use]
extern crate serde_derive;


#[macro_use]
#[cfg(test)]
mod test_helpers;

pub mod constants;
pub mod configuration;
pub mod information;
pub mod error;
mod printer;
mod provider;
mod formatter;
mod sub_command;

use clap::{Arg, App, SubCommand};

use formatter::*;
use printer::*;
use sub_command::*;
use sub_command::SubCommand as FleetSubCommand;


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

    let matches = App::new("fleet")
        .version(constants::PROVIDER_VERSION)
        .author("Daniel Corn <info@cundd.net>")
        .about("Does awesome things")
        //                .arg(Arg::with_name("config")
        //                    .short("c")
        //                    .long("config")
        //                    .value_name("FILE")
        //                    .help("Sets a custom config file")
        //                    .takes_value(true))
        //        .arg(Arg::with_name("INPUT")
        //            .help("Sets the input file to use")
        //            .required(true)
        //            .index(1))
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        //        .arg(format_arg)
        .subcommand(SubCommand::with_name("list")
            .about("Fetch information from all hosts")
            .version(constants::PROVIDER_VERSION)
            .arg(configuration_arg.clone())
            .arg(format_arg.clone())
            .arg(packages_arg.clone())
        )
        .subcommand(SubCommand::with_name("show")
            .about("Fetch information for the given host")
            .version(constants::PROVIDER_VERSION)
            .arg(Arg::with_name("host")
                .short("h")
                .long("host")
                .required(true)
                .value_name("HOST")
                .help("Key of the host's configuration")
                .index(1)
                .takes_value(true))
            .arg(configuration_arg.clone())
            .arg(format_arg.clone())
            .arg(packages_arg.clone())
        )
        .subcommand(SubCommand::with_name("show-packages")
            .about("Show packages for the given host")
            .version(constants::PROVIDER_VERSION)
            .arg(Arg::with_name("host")
                .short("h")
                .long("host")
                .required(true)
                .value_name("HOST")
                .help("Key of the host's configuration")
                .index(1)
                .takes_value(true))
            .arg(configuration_arg.clone())
            .arg(format_arg.clone())
        )
        .subcommand(SubCommand::with_name("provide")
            .about("Use the program as information provider")
            .version(constants::PROVIDER_VERSION)
            .arg(format_arg)
            .arg(Arg::with_name("debug")
                .short("d")
                .help("print debug information verbosely")))
        .get_matches();

    let (subcommand, subcommand_matches_option) = get_subcommand(&matches);

    let default_format = match &subcommand {
        &FleetSubCommand::ProvideCommand(_) => "json",
        _ => "console"
    };

    let formatter = get_formatter(default_format, subcommand_matches_option).unwrap();

    Printer::print_if_error(subcommand.exec(&formatter, subcommand_matches_option));
}

