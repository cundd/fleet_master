use error::Error;
use super::*;

pub struct Factory;

impl Factory {
    pub fn get_formatter(format: &str) -> Result<Box<Formatter>, Error> {
        match format {
            "json" => Ok(Box::new(JsonFormatter {})),
            _ => Err(Error::new(format!("No formatter found for format {}", format)))
        }
    }
}
