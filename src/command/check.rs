use super::{
    ssh_fetch::fetch_information_collection, CommandTrait, DefaultArgs,
};
use crate::{error::Error, prepare_message, FormatterTrait};
use ansi_term::Colour;
use clap::Args;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct CheckArgs {
    #[command(flatten)]
    pub common: DefaultArgs,

    /// Check disabled configuration entries
    #[arg(short, long)]
    pub include_disabled: bool,
}

#[derive(Default)]
pub struct CheckCommand {}

impl CommandTrait for CheckCommand {
    type Args = CheckArgs;

    fn exec<F: FormatterTrait>(
        &self,
        _formatter: &F,
        configuration_file: PathBuf,
        arguments: Self::Args,
    ) -> Result<(), Error> {
        let (information_collection, error_collection) =
            fetch_information_collection(
                configuration_file,
                arguments.include_disabled,
            )?;

        for (host, _) in information_collection {
            println!(
                "{}",
                Colour::Green.paint(format!("[OK] Host \"{}\"", host))
            );
        }
        for (host, error) in error_collection {
            eprintln!(
                "{}",
                Colour::Red.paint(format!(
                    "[ERROR] Host \"{}\": {}",
                    host,
                    prepare_message(error.message())
                ))
            );
        }

        Ok(())
    }
}
