use crate::{
    error::Error,
    information::Platform,
    provider::{LocalProvider, Provider},
    FormatterTrait, Printer,
};
use clap::Args;

#[derive(Args, Debug)]
pub struct ProvideArgs {
    /// Print debug information verbosely
    #[arg(short, long)]
    pub debug: bool,
}

#[derive(Default)]
pub struct ProvideCommand {}

impl ProvideCommand {
    pub fn provide<F: FormatterTrait>(&self, formatter: &F) -> Result<(), Error> {
        let platform = Platform::new_for_current_env();
        let result = LocalProvider::new().get_information();

        Printer::print_result(match result {
            Ok(information) => formatter.format_information(&platform.host, &information, true),
            Err(e) => Err(e),
        });

        Ok(())
    }
}
