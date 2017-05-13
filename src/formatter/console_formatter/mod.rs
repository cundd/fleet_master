mod matrix;
mod table;
mod array;

use error::Error;
use information::*;
use self::table::Table;
use self::matrix::Matrix;
use self::array::map;

pub struct ConsoleFormatter;

const HEADERS: &'static [&'static str] = &[
    "Host", // host
    "Fleet: Protocol", // fleet.protocol
    "Fleet: Provider version", // fleet.provider_version
    "Fleet: Provider name", // fleet.provider_name
    "System Platform Language", // system.platform.language
    "System Platform Version", // system.platform.version
    "System Platform Sapi", // system.platform.sapi
    "System Platform Host", // system.platform.host
    "System Platform OS vendor", // system.platform.os.vendor
    "System Platform OS version", // system.platform.os.version
    "System Platform OS machine", // system.platform.os.machine
    "System Platform OS info", // system.platform.os.info
    "System Application Name", // system.application.name
    "System Application Version", // system.application.version
];

impl super::FormatterTrait for ConsoleFormatter {
    fn format_information(&self, host: &str, information: Result<Information, Error>) -> Result<String, Error> {
        let mut information_collection: InformationCollection = InformationCollection::new();
        information_collection.insert(host.to_owned(), information?);

        let matrix = Matrix::from_information_collection(information_collection);
        Ok(Table::left_header(&matrix))
    }

    fn format_information_collection(&self, information: InformationCollection) -> Result<String, Error> {
        let matrix = Matrix::from_information_collection(information);
        Ok(Table::left_header(&matrix))
    }
}

impl Matrix<String> {
    fn from_information_collection(information_collection: InformationCollection) -> Matrix<String> {
        let mut rows: Vec<Vec<String>> = Vec::with_capacity(information_collection.len() + 1);

        rows.push(map(HEADERS, |x| String::from(x.to_owned())));

        for (host, info) in information_collection {
            let mut cells: Vec<String> = Vec::with_capacity(HEADERS.len());

            cells.push(host);
            cells.push(info.fleet.protocol);
            cells.push(info.fleet.provider_version);
            cells.push(info.fleet.provider_name);
            cells.push(info.system.platform.language);
            cells.push(info.system.platform.version);
            cells.push(info.system.platform.sapi);
            cells.push(info.system.platform.host);
            cells.push(info.system.platform.os.vendor);
            cells.push(info.system.platform.os.version);
            cells.push(info.system.platform.os.machine);
            cells.push(info.system.platform.os.info);
            cells.push(info.system.application.name);
            cells.push(info.system.application.version);

            rows.push(cells);
        }

        Matrix::from_vec(rows)
    }
}
