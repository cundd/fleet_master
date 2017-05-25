use error::Error;
use information::*;
use serde_json;

pub struct JsonFormatter;


impl super::FormatterTrait for JsonFormatter {
    fn format_information(&self, _: &str, information: Result<Information, Error>, show_packages: bool) -> Result<String, Error> {
        let information_no_error = information?;

        let info = if !show_packages {
            information_no_error.without_packages()
        } else {
            information_no_error
        };

        match serde_json::to_string_pretty(&info) {
            Ok(s) => Ok(s),
            Err(e) => Err(Error::from_error(e)),
        }
    }
    fn format_information_collection(&self, information: InformationCollection, show_packages: bool) -> Result<String, Error> {
        let information_collection = if !show_packages {
            collection_without_packages(information)
        } else {
            information
        };

        match serde_json::to_string_pretty(&information_collection) {
            Ok(s) => Ok(s),
            Err(e) => Err(Error::from_error(e)),
        }
    }
}
