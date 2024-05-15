use clap::Args;

use crate::{error::Error, FormatterTrait, Printer};
use std::path::PathBuf;

use super::{ssh_fetch::fetch_information_collection, CommandTrait, DefaultArgs};

#[derive(Args, Debug)]
pub struct ListArgs {
    /// Sets if packages are listed
    #[arg(short, long)]
    pub packages: bool,

    #[command(flatten)]
    pub common: DefaultArgs,
}

#[derive(Default)]
pub struct ListCommand {}

impl CommandTrait for ListCommand {
    type Args = ListArgs;

    fn exec<F: FormatterTrait>(
        &self,
        formatter: &F,
        configuration_file: PathBuf,
        arguments: &ListArgs,
    ) -> Result<(), Error> {
        let (information_collection, _) = fetch_information_collection(configuration_file)?;

        Printer::print_result(
            formatter.format_information_collection(information_collection, arguments.packages),
        );

        Ok(())
    }
}
