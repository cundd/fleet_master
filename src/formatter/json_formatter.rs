use error::Error;
use information::*;
use serde_json;

pub struct JsonFormatter;


impl super::Formatter for JsonFormatter {
    fn format_information(&self, information: Result<Information, Error>) -> Result<String, Error> {
        let information_no_error = information?;

        match serde_json::to_string(&information_no_error) {
            Ok(s) => Ok(s),
            Err(e) => Err(Error::new_from_error(e)),
        }
    }
    fn format_information_collection(&self, information: InformationCollection) -> Result<String, Error> {
        match serde_json::to_string(&information) {
            Ok(s) => Ok(s),
            Err(e) => Err(Error::new_from_error(e)),
        }
    }
}