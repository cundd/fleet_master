use std::slice::Iter;
use std::slice::IterMut;
use error::Error;
use information::*;

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
        let column_widths = calc_column_widths(&information_collection);

        Ok(build_header(&column_widths) + "\n" + &build_body(information_collection, &column_widths))
    }

    fn format_information_collection(&self, information: InformationCollection) -> Result<String, Error> {
        let column_widths = calc_column_widths(&information);

        Ok(build_header(&column_widths) + "\n" + &build_body(information, &column_widths))
    }
}

fn build_body(information_collection: InformationCollection, column_widths: &Vec<usize>) -> String {
    let mut rows: Vec<String> = Vec::with_capacity(information_collection.len() + 1);

    for (host, info) in information_collection {
        let mut iterator: Iter<usize> = column_widths.iter();
        let mut cells: Vec<String> = Vec::with_capacity(HEADERS.len());

        cells.push(format!("{:width$}", host, width = iterator.next().unwrap()));
        cells.push(format!("{:width$}", info.fleet.protocol, width = iterator.next().unwrap()));
        cells.push(format!("{:width$}", info.fleet.provider_version, width = iterator.next().unwrap()));
        cells.push(format!("{:width$}", info.fleet.provider_name, width = iterator.next().unwrap()));
        cells.push(format!("{:width$}", info.system.platform.language, width = iterator.next().unwrap()));
        cells.push(format!("{:width$}", info.system.platform.version, width = iterator.next().unwrap()));
        cells.push(format!("{:width$}", info.system.platform.sapi, width = iterator.next().unwrap()));
        cells.push(format!("{:width$}", info.system.platform.host, width = iterator.next().unwrap()));
        cells.push(format!("{:width$}", info.system.platform.os.vendor, width = iterator.next().unwrap()));
        cells.push(format!("{:width$}", info.system.platform.os.version, width = iterator.next().unwrap()));
        cells.push(format!("{:width$}", info.system.platform.os.machine, width = iterator.next().unwrap()));
        cells.push(format!("{:width$}", info.system.platform.os.info, width = iterator.next().unwrap()));
        cells.push(format!("{:width$}", info.system.application.name, width = iterator.next().unwrap()));
        cells.push(format!("{:width$}", info.system.application.version, width = iterator.next().unwrap()));

        rows.push(cells.join(" | "));
    }

    rows.join("\n")
}

fn build_header(column_widths: &Vec<usize>) -> String {
    let mut iterator: Iter<usize> = column_widths.iter();
    let mut text_cells: Vec<String> = Vec::with_capacity(HEADERS.len());

    let mut rows: Vec<String> = Vec::with_capacity(2);
    for header in HEADERS {
        text_cells.push(format!("{:width$}", header, width = iterator.next().unwrap()));
    }
    rows.push(text_cells.join(" | "));


    let mut underline_cells: Vec<String> = Vec::with_capacity(HEADERS.len());
    for column_width in column_widths {
        underline_cells.push(String::from_utf8(vec![b'-'; *column_width]).unwrap());
    }
    rows.push(underline_cells.join(" | "));

    rows.join("\n")
}

fn calc_column_widths(information_collection: &InformationCollection) -> Vec<usize> {
    let mut column_widths = Vec::with_capacity(HEADERS.len());

    for header in HEADERS {
        column_widths.push(header.len());
    }

    for (host, info) in information_collection {
        let mut iterator: IterMut<usize> = column_widths.iter_mut();

        overwrite_if_bigger(iterator.next(), host.len(), host); // Host
        overwrite_if_bigger(iterator.next(), info.fleet.protocol.len(), "info.fleet.protocol"); // fleet.protocol
        overwrite_if_bigger(iterator.next(), info.fleet.provider_version.len(), "info.fleet.provider_version"); // fleet.provider_version
        overwrite_if_bigger(iterator.next(), info.fleet.provider_name.len(), "info.fleet.provider_name"); // fleet.provider_name
        overwrite_if_bigger(iterator.next(), info.system.platform.language.len(), "info.system.platform.language"); // system.platform.language
        overwrite_if_bigger(iterator.next(), info.system.platform.version.len(), "info.system.platform.version"); // system.platform.version
        overwrite_if_bigger(iterator.next(), info.system.platform.sapi.len(), "info.system.platform.sapi"); // system.platform.sapi
        overwrite_if_bigger(iterator.next(), info.system.platform.host.len(), "info.system.platform.host"); // system.platform.host
        overwrite_if_bigger(iterator.next(), info.system.platform.os.vendor.len(), "info.system.platform.os.vendor"); // system.platform.os.vendor
        overwrite_if_bigger(iterator.next(), info.system.platform.os.version.len(), "info.system.platform.os.version"); // system.platform.os.version
        overwrite_if_bigger(iterator.next(), info.system.platform.os.machine.len(), "info.system.platform.os.machine"); // system.platform.os.machine
        overwrite_if_bigger(iterator.next(), info.system.platform.os.info.len(), "info.system.platform.os.info"); // system.platform.os.info
        overwrite_if_bigger(iterator.next(), info.system.application.name.len(), "info.system.application.name"); // system.application.name
        overwrite_if_bigger(iterator.next(), info.system.application.version.len(), "info.system.application.version"); // system.application.version
    }

    column_widths
}

fn overwrite_if_bigger(orig: Option<&mut usize>, new: usize, descriptor: &str) {
    match orig {
        Some(o) => {
            if new > *o { *o = new }
        }
        None => panic!("not good {}", descriptor),
    }
}
