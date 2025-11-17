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
    #[arg()]
    pub hosts: Vec<String>,

    /// Update all hosts
    #[arg(short = 'a', long)]
    pub update_all: bool,

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
        let hosts = arguments.hosts;
        if hosts.is_empty() && !arguments.update_all {
            return Err(Error::new(
                "At least one host or `update-all` must be given",
            ));
        }

        let (collection, errors) = if !hosts.is_empty() {
            execute_update_for_hosts(configuration_file, &hosts)?
        } else {
            execute_update_for_collection(configuration_file)?
        };

        Printer::print_result(formatter.format_shell_output_collection(collection, errors));

        Ok(())
    }
}
