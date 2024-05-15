use super::{ssh_fetch::fetch_information_collection, CommandTrait, DefaultArgs};
use crate::{error::Error, prepare_message, FormatterTrait};
use ansi_term::Colour;
use clap::Args;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct CheckArgs {
    #[command(flatten)]
    pub common: DefaultArgs,
}

#[derive(Default)]
pub struct CheckCommand {}

impl CommandTrait for CheckCommand {
    type Args = CheckArgs;

    fn exec<F: FormatterTrait>(
        &self,
        _formatter: &F,
        configuration_file: PathBuf,
        _arguments: &CheckArgs,
    ) -> Result<(), Error> {
        let (information_collection, error_collection) =
            match fetch_information_collection(configuration_file) {
                Ok(r) => r,
                Err(e) => return Err(e),
            };

        for (host, _) in information_collection {
            println!("{}", Colour::Green.paint(format!("[OK] Host \"{}\"", host)));
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
