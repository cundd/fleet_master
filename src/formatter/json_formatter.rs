use super::FormatterResult;
use crate::error::*;
use crate::information::*;
use crate::shell::ShellOutputCollection;
use serde::Serialize;

pub struct JsonFormatter;

impl JsonFormatter {
    fn format_data<A: serde::Serialize>(&self, data: A) -> FormatterResult {
        match serde_json::to_string_pretty(&data) {
            Ok(s) => Ok(s),
            Err(e) => Err(Error::from_error(&e)),
        }
    }
}

impl super::FormatterTrait for JsonFormatter {
    fn format_information(
        &self,
        _: &str,
        information: &Information,
        show_packages: bool,
    ) -> FormatterResult {
        if !show_packages {
            self.format_data(information.without_packages())
        } else {
            self.format_data(information)
        }
    }
    fn format_information_collection(
        &self,
        information: InformationCollection,
        show_packages: bool,
    ) -> FormatterResult {
        let information_collection = if !show_packages {
            collection_without_packages(information)
        } else {
            information
        };

        self.format_data(information_collection)
    }

    fn format_packages(&self, packages: &Packages) -> FormatterResult {
        self.format_data(packages)
    }

    fn format_packages_from_information_collection(
        &self,
        information_collection: InformationCollection,
    ) -> FormatterResult {
        let packages_collection: Vec<Packages> = information_collection
            .into_values()
            .map(|i| i.packages)
            .collect();

        self.format_data(packages_collection)
    }

    fn format_shell_output_collection(
        &self,
        outputs: ShellOutputCollection,
        errors: ErrorCollection,
    ) -> FormatterResult {
        #[derive(Serialize)]
        struct ShellOutputJson {
            outputs: ShellOutputCollection,
            errors: Vec<String>,
        }
        self.format_data(ShellOutputJson {
            outputs,
            errors: errors
                .iter()
                .map(|(h, e)| format!("{}: {}", h, e))
                .collect(),
        })
    }
}
