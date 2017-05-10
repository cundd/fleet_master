use std::clone::Clone;
use std::fmt::Display;

//use std::str::replace;

use super::matrix::Matrix;

pub struct Table;

type StringRow<'a> = [&'a str];
type StringMatrix<'a> = [&'a StringRow<'a>];

impl Table {
    pub fn top_header(header: &StringRow, contents: &StringMatrix) -> String {
        let mut rows = vec!(header);
        rows.extend_from_slice(&contents);

        let matrix = Matrix::from_slice(&rows);
        let column_widths = calc_column_widths(&matrix);

        println!("{}", build_layout_top(&column_widths, matrix.clone()));
        build_layout_top(&column_widths, matrix)
    }

    pub fn left_header(header: &StringRow, contents: &StringMatrix) -> String {
        let mut rows = vec!(header);
        rows.extend_from_slice(&contents);

        let matrix = Matrix::from_slice(&rows).transpose();
        let column_widths = calc_column_widths(&matrix);

        println!("{}", build_layout_left(&column_widths, matrix.clone()));
        build_layout_left(&column_widths, matrix)

        //
        //
        //        let mut rows: Vec<&[&str]> = contents.to_owned();
        //        rows.push(header);
        //        let column_widths = calc_column_widths(&rows);
        //        let vector_size = header.len();
        //
        //        let mut text_rows: Vec<String> = Vec::with_capacity(rows.len());
        //        for row in rows {
        //            let mut text_cells: Vec<String> = Vec::with_capacity(vector_size);
        //
        //            for (width, cell) in column_widths.iter().zip(row) {
        //                text_cells.push(format!("{:width$}", cell, width = width));
        //            }
        //            //        println!("{:#?}", column_widths);
        //            //
        //            //        build_header_layout_left(&column_widths, header) + "\n" + &build_body_layout_top(&column_widths, contents)
        //
        //            text_rows.push(text_cells.join(" | "));
        //        }
        //
        //        text_rows.join("\n")
    }
}

fn build_layout_top(column_widths: &[usize], data: Matrix<&str>) -> String {
    let mut output = "".to_owned();

    for (i, row) in data.data.iter().enumerate() {
        output += "|";
        for (cell, width) in row.iter().zip(column_widths) {
            output += &format!(" {:width$} |", cell, width = width);
        }
        output += "\n";

        // Add the line below the headline
        if i == 0 {
            output += "|";
            for width in column_widths {
                output += &format!(" {} |", &String::from_utf8(vec![b'-'; *width]).unwrap());
            }
            output += "\n";
        }
    }

    output
}


fn build_layout_left<T: Clone + Display>(column_widths: &[usize], data: Matrix<T>) -> String {
    let mut output = "".to_owned();

    for row in data.data {
        output += "|";
        for (cell, width) in row.iter().zip(column_widths) {
            output += &format!(" {:width$} |", cell, width = width);
        }
        output += "\n";
    }

    output
}


//fn build_body(information_collection: InformationCollection, column_widths: &Vec<usize>) -> String {
//    let mut rows: Vec<String> = Vec::with_capacity(information_collection.len() + 1);
//
//    for (host, info) in information_collection {
//        let mut iterator: Iter<usize> = column_widths.iter();
//        let mut cells: Vec<String> = Vec::with_capacity(HEADERS.len());
//
//        cells.push(format!("{:width$}", host, width = iterator.next().unwrap()));
//        cells.push(format!("{:width$}", info.fleet.protocol, width = iterator.next().unwrap()));
//        cells.push(format!("{:width$}", info.fleet.provider_version, width = iterator.next().unwrap()));
//        cells.push(format!("{:width$}", info.fleet.provider_name, width = iterator.next().unwrap()));
//        cells.push(format!("{:width$}", info.system.platform.language, width = iterator.next().unwrap()));
//        cells.push(format!("{:width$}", info.system.platform.version, width = iterator.next().unwrap()));
//        cells.push(format!("{:width$}", info.system.platform.sapi, width = iterator.next().unwrap()));
//        cells.push(format!("{:width$}", info.system.platform.host, width = iterator.next().unwrap()));
//        cells.push(format!("{:width$}", info.system.platform.os.vendor, width = iterator.next().unwrap()));
//        cells.push(format!("{:width$}", info.system.platform.os.version, width = iterator.next().unwrap()));
//        cells.push(format!("{:width$}", info.system.platform.os.machine, width = iterator.next().unwrap()));
//        cells.push(format!("{:width$}", info.system.platform.os.info, width = iterator.next().unwrap()));
//        cells.push(format!("{:width$}", info.system.application.name, width = iterator.next().unwrap()));
//        cells.push(format!("{:width$}", info.system.application.version, width = iterator.next().unwrap()));
//
//        rows.push(cells.join(" | "));
//    }
//
//    rows.join("\n")
//}

//fn build_header_layout_top(column_widths: &[usize], header: &StringRow) -> String {
//    let mut iterator: Iter<usize> = column_widths.iter();
//    let vector_size = header.len();
//    let mut text_cells: Vec<String> = Vec::with_capacity(vector_size);
//
//    let mut rows: Vec<String> = Vec::with_capacity(2);
//    for header in header {
//        let width = match iterator.next() {
//            Some(w) => w,
//            None => panic!("No width"),
//        };
//
//        text_cells.push(format!("{:width$}", header, width = width));
//    }
//    rows.push(text_cells.join(" | "));
//
//
//    let mut underline_cells: Vec<String> = Vec::with_capacity(vector_size);
//    for column_width in column_widths {
//        underline_cells.push(String::from_utf8(vec![b'-'; *column_width]).unwrap());
//    }
//    rows.push(underline_cells.join(" | "));
//
//    rows.join("\n")
//}

//fn build_body_layout_top(column_widths: &[usize], contents: &StringMatrix) -> String {
//    let mut rows: Vec<String> = Vec::with_capacity(contents.len() + 1);
//    let column_count = column_widths.len();
//
//    for &row_content in contents {
//        let mut iterator: Iter<usize> = column_widths.iter();
//        let mut cells: Vec<String> = Vec::with_capacity(column_count);
//
//        for cell_content in row_content {
//            cells.push(format!("{:width$}", cell_content, width = iterator.next().unwrap()));
//        }
//
//        rows.push(cells.join(" | "));
//    }
//
//    rows.join("\n")
//}

//fn calc_column_widths_with_header(header: &StringRow, contents: &StringMatrix) -> Vec<usize> {
//    let mut column_widths = Vec::with_capacity(header.len());
//
//    for header_column in header {
//        column_widths.push(header_column.len());
//    }
//
//    for row in contents {
//        for (cell, previous_value) in row.iter().zip(column_widths.iter_mut()) {
//            let new = cell.len();
//            if new > *previous_value { *previous_value = new }
//        }
//    }
//
//    column_widths
//}

fn calc_column_widths(matrix: &Matrix<&str>) -> Vec<usize> {
    let first_row = matrix.data.first();
    if first_row.is_none() {
        return vec![];
    }

    let mut column_widths = vec![0; first_row.unwrap().len()];

    for row in &matrix.data {
        for (cell, previous_value) in row.iter().zip(column_widths.iter_mut()) {
            let new = cell.len();
            if new > *previous_value { *previous_value = new }
        }
    }

    column_widths
}

//fn overwrite_if_bigger(iterator: &mut IterMut<usize>, new: usize, descriptor: &str) {
//    match iterator.next() {
//        Some(o) => {
//            if new > *o { *o = new }
//        }
//        None => panic!("Column count mismatch"),
//    }
//}

#[cfg(test)]
mod tests {
    use super::*;

    //    fn build_matrix<'a>() -> Vec<&'a StringRow<'a>> {
    //        let row1 = vec!("Apple", "Red");
    //        let row2 = vec!("Pear", "Green");
    //        let row3 = vec!("Banana", "Yellow");
    //        let row4 = vec!("Orange", "Orange");
    //
    //        vec!(
    //            row1.as_slice(),
    //            row2.as_slice(),
    //            row3.as_slice(),
    //            row4.as_slice(),
    //            )
    //    }

    #[test]
    fn top_header_test() {
        let header: &StringRow = &["Fruit", "Color"];

        let row1 = vec!("Apple", "Red");
        let row2 = vec!("Pear", "Green");
        let row3 = vec!("Banana", "Yellow");
        let row4 = vec!("Orange", "Orange");

        let content = vec!(
                          row1.as_slice(),
                          row2.as_slice(),
                          row3.as_slice(),
                          row4.as_slice(),
                          );

        let output = Table::top_header(header, &content);
        let expected = "| Fruit  | Color  |\n| ------ | ------ |\n| Apple  | Red    |\n| Pear   | Green  |\n| Banana | Yellow |\n| Orange | Orange |\n";

        // println!("A: {}", expected.replace("\n", "\\n"));
        // println!("B: {}", output.replace("\n", "\\n"));

        assert_eq!(expected, output);
    }

    #[test]
    fn left_header_test() {
        let header: &StringRow = &["Fruit", "Color"];
        let row1 = vec!("Apple", "Red");
        let row2 = vec!("Pear", "Green");
        let row3 = vec!("Banana", "Yellow");
        let row4 = vec!("Orange", "Orange");

        let content = vec!(
                          row1.as_slice(),
                          row2.as_slice(),
                          row3.as_slice(),
                          row4.as_slice(),
                          );
        let output = Table::left_header(header, &content);

        let expected = r"| Fruit | Apple | Pear  | Banana | Orange |
| Color | Red   | Green | Yellow | Orange |
";
        // println!("A: {}", expected.replace("\n", "\\n"));
        // println!("B: {}", output.replace("\n", "\\n"));

        assert_eq!(expected, output);
    }
}

