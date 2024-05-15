use super::matrix::*;
use std::clone::Clone;
use std::fmt::Debug;
use std::fmt::Display;

pub struct Table;

impl Table {
    pub fn top_header<S>(input: &Matrix<S>) -> String
    where
        S: Into<String> + Clone + Display + Debug,
    {
        let column_widths = calc_column_widths(input);
        build_layout_top(&column_widths, input)
    }

    pub fn left_header<S>(input: &Matrix<S>) -> String
    where
        S: Into<String> + Clone + Display + Debug,
    {
        let transposed = input.transpose();
        let column_widths = calc_column_widths(&transposed);
        build_layout_left(&column_widths, &transposed)
    }
}

fn build_layout_top<S>(column_widths: &[usize], input: &Matrix<S>) -> String
where
    S: Into<String> + Clone + Display + Debug,
{
    let mut output = "".to_owned();

    for (i, row) in input.data().iter().enumerate() {
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

fn build_layout_left<S>(column_widths: &[usize], input: &Matrix<S>) -> String
where
    S: Into<String> + Clone + Display + Debug,
{
    let mut output = "".to_owned();

    for row in input.data().iter() {
        output += "|";
        for (cell, width) in row.iter().zip(column_widths) {
            output += &format!(" {:width$} |", cell, width = width);
        }
        output += "\n";
    }

    output
}

fn calc_column_widths<S>(input: &Matrix<S>) -> Vec<usize>
where
    S: Into<String> + Clone + Debug + Display,
{
    let first_row = input.data().first();
    if first_row.is_none() {
        return vec![];
    }

    let mut column_widths = vec![0; first_row.unwrap().len()];

    for row in input.data().iter() {
        for (cell, previous_value) in row.iter().zip(column_widths.iter_mut()) {
            let new = format!("{}", cell).len();
            if new > *previous_value {
                *previous_value = new
            }
        }
    }

    column_widths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn top_header_test() {
        let content = vec![
            vec!["Fruit", "Color"],
            vec!["Apple", "Red"],
            vec!["Pear", "Green"],
            vec!["Banana", "Yellow"],
            vec!["Orange", "Orange"],
        ];

        let matrix = Matrix::from_vec(content);
        let output = Table::top_header(&matrix);
        let expected = "| Fruit  | Color  |\n| ------ | ------ |\n| Apple  | Red    |\n| Pear   | Green  |\n| Banana | Yellow |\n| Orange | Orange |\n";

        // println!("A: {}", expected.replace("\n", "\\n"));
        // println!("B: {}", output.replace("\n", "\\n"));

        assert_eq!(expected, output);
    }

    #[test]
    fn left_header_test() {
        let content = vec![
            vec!["Fruit", "Color"],
            vec!["Apple", "Red"],
            vec!["Pear", "Green"],
            vec!["Banana", "Yellow"],
            vec!["Orange", "Orange"],
        ];

        let matrix = Matrix::from_vec(content);
        let output = Table::left_header(&matrix);

        let expected = r"| Fruit | Apple | Pear  | Banana | Orange |
| Color | Red   | Green | Yellow | Orange |
";
        // println!("A: {}", expected.replace("\n", "\\n"));
        // println!("B: {}", output.replace("\n", "\\n"));

        assert_eq!(expected, output);
    }
}
