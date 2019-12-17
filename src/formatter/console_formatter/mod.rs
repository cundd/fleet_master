#[allow(dead_code)]
mod matrix;
mod table;

use std::collections::BTreeMap;

use crate::information::*;
use self::table::Table;
use self::matrix::Matrix;

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
    // "System Platform OS info", // system.platform.os.info
    "System Application Name", // system.application.name
    "System Application Version", // system.application.version
];

const PACKAGE_HEADERS: &'static [&'static str] = &[
    "Key",
    "Version",
    "State",
    "Description",
];

impl super::FormatterTrait for ConsoleFormatter {
    fn format_information(&self, host: &str, information: &Information, show_packages: bool) -> super::FormatterResult {
        let mut information_collection: InformationCollection = InformationCollection::new();
        information_collection.insert(host.to_owned(), information.clone());

        let matrix = Matrix::from_information_collection(information_collection, show_packages);
        Ok(Table::left_header(&matrix))
    }

    fn format_information_collection(&self, information: InformationCollection, show_packages: bool) -> super::FormatterResult {
        let matrix = Matrix::from_information_collection(information, show_packages);
        Ok(Table::left_header(&matrix))
    }

    fn format_packages(&self, packages: &Packages) -> super::FormatterResult {
        let matrix = Matrix::from_packages(packages);
        Ok(Table::top_header(&matrix))
    }

    fn format_packages_from_information_collection(&self, information_collection: InformationCollection) -> super::FormatterResult {
        let mut output = "".to_owned();
        for (host, information) in information_collection {
            if information.packages.len() > 0 {
                output += &(format!("Packages of host '{}':\n", host));
                let matrix = Matrix::from_packages(&information.packages);
                output += &Table::top_header(&matrix);
                output += "\n\n";
            } else {
                output += &(format!("No packages found for host '{}'\n\n", host));
            }
        }

        Ok(output)
    }
}

fn crop_cell_content(content: &str) -> String {
    if content.len() > 50 {
        return String::from(&content[0..49]) + "…";
    }
    String::from(content)
}

impl Matrix<String> {
    #[allow(unused)]
    fn from_information_collection(information_collection: InformationCollection, show_packages: bool) -> Matrix<String> {
        let mut rows: Vec<Vec<String>> = Vec::with_capacity(information_collection.len() + 1);

        rows.push(HEADERS.iter().map(|x| String::from(x.to_owned())).collect());

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
            // cells.push(info.system.platform.os.info);
            cells.push(info.system.application.name);
            cells.push(info.system.application.version);

            rows.push(cells);
        }

        Matrix::from_vec(rows)
    }

    fn from_packages(packages: &Packages) -> Matrix<String> {
        let mut rows: Vec<Vec<String>> = Vec::with_capacity(packages.len() + 1);

        rows.push(PACKAGE_HEADERS.iter().map(|x| String::from(x.to_owned())).collect());

        let all_packages: BTreeMap<_, _> = packages.iter().collect();
        for (_, package) in all_packages {
            let mut cells: Vec<String> = Vec::with_capacity(PACKAGE_HEADERS.len());
            cells.push(package.key.to_owned());
            cells.push(package.version.to_owned());
            cells.push(package.state.to_owned());
            cells.push(crop_cell_content(&package.description));

            rows.push(cells);
        }

        Matrix::from_vec(rows)
    }
}
