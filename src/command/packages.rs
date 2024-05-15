use super::{
    ssh_fetch::{fetch_information_collection, fetch_information_for_hosts},
    CommandTrait, DefaultArgs,
};
use crate::{error::Error, FormatterTrait, Printer};
use clap::Args;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct PackagesArgs {
    /// Comma separated list hosts to fetch the packages
    #[arg(num_args(0..))]
    pub hosts: Vec<String>,

    #[command(flatten)]
    pub common: DefaultArgs,
}

#[derive(Default)]
pub struct PackagesCommand {}

impl CommandTrait for PackagesCommand {
    type Args = PackagesArgs;

    fn exec<F: FormatterTrait>(
        &self,
        formatter: &F,
        configuration_file: PathBuf,
        arguments: &PackagesArgs,
    ) -> Result<(), Error> {
        let hosts = &arguments.hosts;

        let (information_collection, _) = if hosts.is_empty() {
            fetch_information_collection(configuration_file)?
        } else {
            fetch_information_for_hosts(configuration_file, hosts)?
        };

        Printer::print_result(
            formatter.format_packages_from_information_collection(information_collection),
        );

        Ok(())
    }
}
