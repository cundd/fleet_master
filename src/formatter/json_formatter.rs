use serde;
use serde_json;

use error::*;
use information::*;
use super::FormatterResult;

pub struct JsonFormatter;

impl JsonFormatter {
    fn format_data<A: serde::Serialize>(&self, data: &A) -> FormatterResult {
        match serde_json::to_string_pretty(&data) {
            Ok(s) => Ok(s),
            Err(e) => Err(Error::from_error(e)),
        }
    }
}


impl super::FormatterTrait for JsonFormatter {
    fn format_information(&self, _: &str, information: Result<Information, Error>, show_packages: bool) -> FormatterResult {
        let information_no_error = information?;

        let info = if !show_packages {
            information_no_error.without_packages()
        } else {
            information_no_error
        };

        self.format_data(&info)
    }
    fn format_information_collection(&self, information: InformationCollection, show_packages: bool) -> FormatterResult {
        let information_collection = if !show_packages {
            collection_without_packages(information)
        } else {
            information
        };

        self.format_data(&information_collection)
    }

    fn format_packages(&self, information: Information) -> FormatterResult {
        self.format_data(&information.packages)
    }

    fn format_packages_from_information_collection(&self, information_collection: InformationCollection) -> FormatterResult {
        let packages_collection: Vec<Packages> = information_collection.into_iter().map(|(_, i)| i.packages).collect();

        self.format_data(&packages_collection)
    }
}
