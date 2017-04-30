use clap::ArgMatches;
use error::Error;
use information::Platform;
use formatter::*;
use printer::Printer;
use provider::*;
use sub_command::SubCommandTrait;

pub struct ProvideCommand;

impl SubCommandTrait for ProvideCommand {
    fn exec<F: FormatterTrait>(&self, formatter: &F, _: Option<&ArgMatches>) -> Result<(), Error> {
        let platform = Platform::new_for_current_env();
        Printer::print_result(
            formatter.format_information(&platform.host, LocalProvider::new().get_information())
        );

        Ok(())
    }
}
