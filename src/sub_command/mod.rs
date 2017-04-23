mod list_command;
mod show_command;

use clap::ArgMatches;
use error::Error;
use formatter::Formatter;
pub use self::list_command::ListCommand;
pub use self::show_command::ShowCommand;

pub trait SubCommand {
    fn exec(&self, formatter: &Formatter, matches: &ArgMatches) -> Result<(), Error>;
}
