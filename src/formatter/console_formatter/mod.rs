#[allow(dead_code)]
mod matrix;
mod table;
use self::matrix::Matrix;
use self::table::Table;
use crate::information::*;
use crate::shell::ShellOutputCollection;
use std::collections::BTreeMap;
use std::io::IsTerminal;

use super::ErrorCollection;

pub struct ConsoleFormatter {
    pub use_colors: bool,
}

impl ConsoleFormatter {
    pub fn new(use_colors: bool) -> Self {
        Self { use_colors }
    }
}

const HEADERS: &[&str] = &[
    "Host",
    "App Name",
    "App Version",
    "App Install Mode",
    "Provider",
    "Lang",
    "Version",
    "Sapi",
    "Host",
    "OS",
];

const PACKAGE_HEADERS: &[&str] = &["Key", "Version", "State", "Description"];

impl super::FormatterTrait for ConsoleFormatter {
    fn format_information(
        &self,
        host: &str,
        information: &Information,
        show_packages: bool,
    ) -> super::FormatterResult {
        let mut information_collection: InformationCollection = InformationCollection::new();
        information_collection.insert(host.to_owned(), information.clone());

        let matrix = Matrix::from_information_collection(information_collection, show_packages);
        Ok(Table::left_header(&matrix, self.use_colors))
    }

    fn format_information_collection(
        &self,
        information: InformationCollection,
        show_packages: bool,
    ) -> super::FormatterResult {
        let matrix = Matrix::from_information_collection(information, show_packages);
        Ok(Table::top_header(&matrix, self.use_colors))
    }

    fn format_packages(&self, packages: &Packages) -> super::FormatterResult {
        let matrix = Matrix::from_packages(packages);
        Ok(Table::top_header(&matrix, self.use_colors))
    }

    fn format_packages_from_information_collection(
        &self,
        information_collection: InformationCollection,
    ) -> super::FormatterResult {
        let mut output = "".to_owned();
        let is_terminal = std::io::stdin().is_terminal();
        for (host, information) in information_collection {
            if !information.packages.is_empty() {
                output += &(format!("Packages of host '{}':\n", host));
                let matrix = Matrix::from_packages(&information.packages);
                output += &Table::top_header(&matrix, is_terminal);
                output += "\n\n";
            } else {
                output += &(format!("No packages found for host '{}'\n\n", host));
            }
        }

        Ok(output)
    }

    fn format_shell_output_collection(
        &self,
        outputs: ShellOutputCollection,
        errors: ErrorCollection,
    ) -> super::FormatterResult {
        let mut sorted_rows = BTreeMap::new();

        for (host, error) in errors {
            sorted_rows.insert(host, ("Error".to_string(), error.to_string()));
        }
        for (host, output) in outputs {
            sorted_rows.insert(host, ("Ok".to_string(), output));
        }

        use comfy_table::Table;
        let mut table = Table::new();
        table.load_preset(comfy_table::presets::UTF8_FULL);
        table.set_header(vec!["Host", "Output", "Type"]);
        for (host, (result_type, content)) in sorted_rows {
            table.add_row(vec![host, content, result_type]);
        }

        Ok(table.to_string())
    }
}

fn crop_cell_content(content: &str) -> String {
    if content.len() > 50 {
        return String::from(&content[0..49]) + "…";
    }
    String::from(content)
}

impl Matrix<String> {
    fn from_information_collection(
        information_collection: InformationCollection,
        _show_packages: bool,
    ) -> Matrix<String> {
        let mut rows: Vec<Vec<String>> = Vec::with_capacity(information_collection.len() + 1);

        rows.push(HEADERS.iter().map(|x| String::from(x.to_owned())).collect());

        for (host, info) in information_collection {
            let mut cells: Vec<String> = Vec::with_capacity(HEADERS.len());

            cells.push(host);
            cells.push(info.system.application.name);
            cells.push(info.system.application.version);
            cells.push(info.system.application.install_mode.unwrap_or_default());
            cells.push(format!(
                "{} ({})",
                info.fleet.provider_name, info.fleet.provider_version
            ));
            cells.push(info.system.platform.language);
            cells.push(info.system.platform.version);
            cells.push(info.system.platform.sapi);
            cells.push(info.system.platform.host);
            cells.push(format!(
                "{} ({} {})",
                info.system.platform.os.vendor,
                info.system.platform.os.version,
                info.system.platform.os.machine
            ));

            rows.push(cells);
        }

        Matrix::from_vec(rows)
    }

    fn from_packages(packages: &Packages) -> Matrix<String> {
        let mut rows: Vec<Vec<String>> = Vec::with_capacity(packages.len() + 1);

        rows.push(
            PACKAGE_HEADERS
                .iter()
                .map(|x| String::from(x.to_owned()))
                .collect(),
        );

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
