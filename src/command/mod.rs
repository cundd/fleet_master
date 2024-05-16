pub mod check;
pub mod list;
pub mod packages;
pub mod provide;
pub mod search;
pub mod show;
mod ssh_fetch;

use crate::{error::Error, FormatterTrait};
pub use check::{CheckArgs, CheckCommand};
pub use list::{ListArgs, ListCommand};
pub use packages::{PackagesArgs, PackagesCommand};
pub use provide::{ProvideArgs, ProvideCommand};
pub use search::{SearchArgs, SearchCommand};
pub use show::{ShowArgs, ShowCommand};

use clap::Args;
use std::path::PathBuf;

pub trait CommandTrait {
    type Args;

    fn exec<F: FormatterTrait>(
        &self,
        formatter: &F,
        configuration_file: PathBuf,
        arguments: Self::Args,
    ) -> Result<(), Error>;
}

#[derive(Args, Debug)]
pub struct DefaultArgs {
    /// Set the configuration file to read
    #[arg(short, long, value_parser=clap::value_parser!(PathBuf))]
    pub config: Option<PathBuf>,

    /// The output format
    #[arg(short, long)]
    pub format: Option<String>,
}
