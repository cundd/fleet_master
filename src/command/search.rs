use super::{ssh_fetch::fetch_information_collection, CommandTrait, DefaultArgs};
use crate::{
    error::Error,
    filter::{InformationCollectionFilter, PackageFilter},
    FormatterTrait, Printer,
};
use clap::Args;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct SearchArgs {
    /// Package name to search for
    pub package: String,

    /// Only show exact matches
    #[arg(short, long)]
    pub exact: bool,

    #[command(flatten)]
    pub common: DefaultArgs,
}

#[derive(Default)]
pub struct SearchCommand {}

impl CommandTrait for SearchCommand {
    type Args = SearchArgs;

    fn exec<F: FormatterTrait>(
        &self,
        formatter: &F,
        configuration_file: PathBuf,
        arguments: &SearchArgs,
    ) -> Result<(), Error> {
        let package = &arguments.package;
        let exact = arguments.exact;

        if package.trim().is_empty() {
            return Err(Error::new("Please specify the 'package' argument"));
        }

        let (information_collection, _) = fetch_information_collection(configuration_file)?;
        let filtered_collection =
            InformationCollectionFilter::filter_by_package(information_collection, package, exact);
        for (host, information) in filtered_collection {
            Printer::print_result(formatter.format_information(&host, &information, false));
            Printer::print_result(formatter.format_packages(&PackageFilter::filter(
                information.packages,
                package,
                exact,
            )));
        }
        Ok(())
    }
}
