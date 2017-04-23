extern crate serde;
extern crate serde_json;
extern crate serde_yaml;
extern crate ssh2;
extern crate clap;

#[macro_use]
extern crate serde_derive;

pub mod constants;
pub mod configuration;
pub mod information;
pub mod error;
mod printer;
mod provider;
mod formatter;
mod sub_command;

use clap::{Arg, App, SubCommand};

use provider::*;
use formatter::*;
use printer::*;
use sub_command::*;
use sub_command::SubCommand as FleetSubCommand;

#[cfg(test)]
mod test_helpers;

fn main() {
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
        .arg(Arg::with_name("format")
            .short("f")
            .long("format")
            .value_name("FORMAT")
            .takes_value(true)
            .help("Defines the output format"))
        .subcommand(SubCommand::with_name("list")
            .about("Fetch information from all hosts")
            .version(constants::PROVIDER_VERSION)
            .arg(Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .required(true)
                .help("Sets the configuration file to read")
                .takes_value(true)))
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
            .arg(Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .required(true)
                .help("Sets the configuration file to read")
                .takes_value(true))
        )
        .subcommand(SubCommand::with_name("provide")
            .about("Use the program as information provider")
            .version(constants::PROVIDER_VERSION)
            .arg(Arg::with_name("debug")
                .short("d")
                .help("print debug information verbosely")))
        .get_matches();

    let format = matches.value_of("format").unwrap_or("json");
    let formatter_box = Factory::get_formatter(format).unwrap();
    let formatter = &*formatter_box as &Formatter;

    //    // Vary the output based on how many times the user used the "verbose" flag
    //    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    //    match matches.occurrences_of("v") {
    //        0 => println!("No verbose info"),
    //        1 => println!("Some verbose info"),
    //        2 => println!("Tons of verbose info"),
    //        3 | _ => println!("Don't be crazy"),
    //    }

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    if let Some(_) = matches.subcommand_matches("provide") {
        Printer::print_result(
            formatter.format_information(LocalProvider::new().get_information())
        );

        return;
    }

    if let Some(matches) = matches.subcommand_matches("list") {
        let formatter = &*formatter_box as &Formatter;
        let sub_command = ListCommand {};
        Printer::print_error_if_not_ok(sub_command.exec(formatter, &matches));


        return;
    }
    if let Some(matches) = matches.subcommand_matches("show") {
        let formatter = &*formatter_box as &Formatter;
        let sub_command = ShowCommand {};
        Printer::print_error_if_not_ok(sub_command.exec(formatter, &matches));

        return;
    }

    println!("TODO: Print the help");
    //    matches.help();
    // more program logic goes here... println!("Hello, world!");
}


