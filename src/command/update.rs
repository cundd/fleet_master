use super::{
    ssh_execute_shell::{execute_update_for_collection, execute_update_for_hosts},
    CommandTrait, DefaultArgs,
};
use crate::{error::Error, FormatterTrait, Printer};
use clap::Args;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct UpdateArgs {
    /// Hosts to fetch the packages
    #[arg(long = "host", value_name = "host")]
    pub hosts: Option<Vec<String>>,

    #[command(flatten)]
    pub common: DefaultArgs,
}

#[derive(Default)]
pub struct UpdateCommand {}

impl CommandTrait for UpdateCommand {
    type Args = UpdateArgs;

    fn exec<F: FormatterTrait>(
        &self,
        formatter: &F,
        configuration_file: PathBuf,
        arguments: Self::Args,
    ) -> Result<(), Error> {
        let (collection, errors) = match arguments.hosts {
            Some(hosts) => execute_update_for_hosts(configuration_file, &hosts)?,
            None => execute_update_for_collection(configuration_file)?,
        };

        Printer::print_result(formatter.format_shell_output_collection(collection, errors));

        Ok(())
    }
}
