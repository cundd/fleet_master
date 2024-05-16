use super::{
    ssh_execute_shell::{execute_shell_for_collection, execute_shell_for_hosts},
    CommandTrait, DefaultArgs,
};
use crate::{error::Error, FormatterTrait, Printer};
use clap::Args;
use std::path::PathBuf;

#[derive(Args, Debug)]
#[clap(trailing_var_arg = true)]
pub struct ExecArgs {
    /// Shell command to execute
    pub command: Vec<String>,

    /// Hosts to fetch the packages
    #[arg(long = "host", value_name = "host")]
    pub hosts: Option<Vec<String>>,

    #[command(flatten)]
    pub common: DefaultArgs,
}

#[derive(Default)]
pub struct ExecCommand {}

impl CommandTrait for ExecCommand {
    type Args = ExecArgs;

    fn exec<F: FormatterTrait>(
        &self,
        formatter: &F,
        configuration_file: PathBuf,
        arguments: Self::Args,
    ) -> Result<(), Error> {
        let command_string = arguments.command.join(" ");
        let (collection, errors) = match arguments.hosts {
            Some(hosts) => execute_shell_for_hosts(configuration_file, &hosts, command_string)?,
            None => execute_shell_for_collection(configuration_file, command_string)?,
        };

        Printer::print_result(formatter.format_shell_output_collection(collection, errors));

        Ok(())
    }
}
