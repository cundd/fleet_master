use super::{ssh_fetch::fetch_information_for_host, CommandTrait, DefaultArgs};
use crate::{error::Error, FormatterTrait, Printer};
use clap::Args;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct ShowArgs {
    /// Key of the host's configuration
    pub host: String,

    /// Set if packages are listed
    #[arg(short, long)]
    pub packages: bool,

    #[command(flatten)]
    pub common: DefaultArgs,
}

#[derive(Default)]
pub struct ShowCommand {}

impl CommandTrait for ShowCommand {
    type Args = ShowArgs;

    fn exec<F: FormatterTrait>(
        &self,
        formatter: &F,
        configuration_file: PathBuf,
        arguments: Self::Args,
    ) -> Result<(), Error> {
        let host = &arguments.host;
        let information = fetch_information_for_host(configuration_file, host)?;

        Printer::print_result(formatter.format_information(host, &information, false));
        if arguments.packages {
            Printer::print_result(formatter.format_packages(&information.packages));
        }

        Ok(())
    }
}
