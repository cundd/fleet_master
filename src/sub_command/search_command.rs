use clap::ArgMatches;
use error::Error;
use formatter::*;
use printer::Printer;
use sub_command::SubCommandTrait;
use sub_command::SshCommandTrait;
use filter::InformationCollectionFilter;
use filter::PackageFilter;

pub struct SearchCommand;

impl SshCommandTrait for SearchCommand {}

impl SubCommandTrait for SearchCommand {
    fn exec<F: FormatterTrait>(&self, formatter: &F, subcommand_matches_option: Option<&ArgMatches>) -> Result<(), Error> {
        let package = subcommand_matches_option.unwrap().value_of("package");
        let exact = subcommand_matches_option.unwrap().is_present("exact");

        match package {
            Some(package) => {
                let (information_collection, _) = self.fetch_information_collection(subcommand_matches_option)?;
                let filtered_collection = InformationCollectionFilter::filter_by_package(information_collection, package, exact);
                for (host, information) in filtered_collection {
                    Printer::print_result(formatter.format_information(&host, &information, false));
                    Printer::print_result(formatter.format_packages(&PackageFilter::filter(information.packages, package, exact)));
                }
                Ok(())
            }
            None => Err(Error::new("Please specify the 'package' argument"))
        }
    }
}
