#[allow(dead_code)]
use std::fmt::Debug;
use std::iter::IntoIterator;
use std::vec::IntoIter as VecIntoIter;

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix<T> where T: Clone {
    rows: usize,
    columns: usize,
    data: Vec<Vec<T>>,
}

impl<T> Matrix<T> where T: Clone + Debug {
    pub fn new() -> Matrix<T> {
        Matrix {
            rows: 0,
            columns: 0,
            data: vec!(),
        }
    }

    pub fn from_vec(input: Vec<Vec<T>>) -> Matrix<T> {
        let columns = match input.first() {
            Some(row) => row.len(),
            None => 0,
        };

        Matrix {
            rows: input.len(),
            columns: columns,
            data: input,
        }
    }

    #[allow(unused)]
    pub fn from_vec_ref(input: &Vec<Vec<T>>) -> Matrix<T> {
        unimplemented!()
    }

    pub fn from_slice(input: &[&[T]]) -> Matrix<T> {
        Matrix::from_vec(input.iter().map(|x| x.to_vec()).collect())
    }

    pub fn data(&self) -> &Vec<Vec<T>> {
        &self.data
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn columns(&self) -> usize {
        self.columns
    }

    pub fn is_empty(&self) -> bool {
        self.rows == 0 && self.columns == 0
    }

    pub fn transpose(&self) -> Matrix<T> {
        if self.is_empty() {
            return Matrix::new();
        }

        let mut new_data: Vec<Vec<T>> = Vec::with_capacity(self.columns);
        // Prepare the new rows
        for _ in 0..self.columns {
            let new_row: Vec<T> = Vec::with_capacity(self.rows);
            new_data.push(new_row);
        }

        for new_row_index in 0..self.columns {
            let ref mut new_row = new_data[new_row_index];
            for new_column_index in 0..self.rows {
                new_row.push(self.data[new_column_index][new_row_index].clone())
            }
        }

        Matrix {
            rows: self.columns,
            columns: self.rows,
            data: new_data,
        }
    }

    pub fn push_row(&mut self, row: Vec<T>) {
        self.data.push(row);
        self.rows += 1;
    }

    pub fn prepend_row(&mut self, row: Vec<T>) {
        self.data.insert(0, row);
        self.rows += 1;
    }
}


impl<T: Clone> IntoIterator for Matrix<T> {
    type Item = Vec<T>;

    type IntoIter = VecIntoIter<Vec<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn build_data() -> Vec<Vec<&'static str>> {
        vec!(
            vec!("Apple", "Red"),
            vec!("Pear", "Green"),
            vec!("Banana", "Yellow"),
            vec!("Orange", "Orange"),
        )
    }

    #[test]
    fn from_vec_test() {
        let m = Matrix::from_vec(build_data());

        assert_eq!(4, m.rows);
        assert_eq!(2, m.columns);
        assert_eq!(&build_data(), m.data());
    }

    #[test]
    fn from_slice_test() {
        let apple = vec!("Apple", "Red");
        let pear = vec!("Pear", "Green");
        let banana = vec!("Banana", "Yellow");
        let orange = vec!("Orange", "Orange");

        let data = vec!(
            apple.as_slice(),
            pear.as_slice(),
            banana.as_slice(),
            orange.as_slice(),
        );
        let m = Matrix::from_slice(&data);

        assert_eq!(4, m.rows);
        assert_eq!(2, m.columns);
        assert_eq!(&build_data(), m.data());
    }

    //    #[test]
    //    fn from_vec_ref_test() {
    //        let m = Matrix::from_vec_ref(&build_data());
    //
    //        assert_eq!(4, m.rows);
    //        assert_eq!(2, m.columns);
    //        assert_eq!(build_data(), m.data());
    //    }

    #[test]
    fn data_test() {
        let m = Matrix::from_vec(build_data());

        assert_eq!(
            vec!(
                vec!("Apple", "Red"),
                vec!("Pear", "Green"),
                vec!("Banana", "Yellow"),
                vec!("Orange", "Orange"),
            ),
            *m.data()
        );
    }

    #[test]
    fn rows_test() {
        let m = Matrix::from_vec(build_data());

        assert_eq!(4, m.rows());
        assert_eq!(m.rows, m.rows());
    }

    #[test]
    fn columns_test() {
        let m = Matrix::from_vec(build_data());

        assert_eq!(2, m.columns());
        assert_eq!(m.columns, m.columns());
    }

    #[test]
    fn into_iter_test() {
        let m = Matrix::from_vec(build_data());

        let iterator = m.into_iter();
        for (i, row) in iterator.enumerate() {
            match i {
                0 => assert_eq!(vec!("Apple", "Red"), row),
                1 => assert_eq!(vec!("Pear", "Green"), row),
                2 => assert_eq!(vec!("Banana", "Yellow"), row),
                3 => assert_eq!(vec!("Orange", "Orange"), row),
                _ => panic!("Unexpected iteration")
            }
        }
    }

    #[test]
    fn for_test() {
        let m = Matrix::from_vec(build_data());
        let mut i = 0;
        for row in m {
            match i {
                0 => assert_eq!(vec!("Apple", "Red"), row),
                1 => assert_eq!(vec!("Pear", "Green"), row),
                2 => assert_eq!(vec!("Banana", "Yellow"), row),
                3 => assert_eq!(vec!("Orange", "Orange"), row),
                _ => panic!("Unexpected iteration")
            }

            i += 1;
        }
    }

    #[test]
    fn transpose_u8_test() {
        let m = Matrix::from_vec(
            vec!(
                vec!(11, 12, 13, 14),
                vec!(21, 22, 23, 24),
            )
        );
        let expected_rotated = Matrix::from_vec(
            vec!(
                vec!(11, 21),
                vec!(12, 22),
                vec!(13, 23),
                vec!(14, 24),
            )
        );

        assert_eq!(expected_rotated, m.transpose());
    }

    #[test]
    fn transpose_str_test() {
        let m = Matrix::from_vec(build_data());
        let expected_rotated = Matrix::from_vec(
            vec!(
                vec!("Apple", "Pear", "Banana", "Orange"),
                vec!("Red", "Green", "Yellow", "Orange"),
            )
        );


        assert_eq!(expected_rotated, m.transpose());
    }

    #[test]
    fn transpose_matrix_empty_test() {
        let m = Matrix::<&str>::new();

        let rotated = m.transpose();
        assert!(rotated.is_empty());
        assert_eq!(Matrix::new(), rotated);
    }

    #[test]
    fn new_is_empty_test() {
        assert_eq!(Matrix::<u8>::new(), Matrix::from_vec(vec!()));
        assert!(Matrix::<u8>::new().is_empty());
        assert!(Matrix::<u8>::from_vec(vec!()).is_empty());

        assert_eq!(Matrix::<&str>::new(), Matrix::from_vec(vec!()));
        assert!(Matrix::<&str>::new().is_empty());
        assert!(Matrix::<&str>::from_vec(vec!()).is_empty());
    }

    #[test]
    fn push_row_test() {
        let mut m = Matrix::from_vec(build_data());

        m.push_row(vec!("Grape", "Blue"));

        assert_eq!(
            vec!(
                vec!("Apple", "Red"),
                vec!("Pear", "Green"),
                vec!("Banana", "Yellow"),
                vec!("Orange", "Orange"),
                vec!("Grape", "Blue"),
            ),
            *m.data()
        );
        assert_eq!(5, m.rows);
    }

    #[test]
    fn prepend_row_test() {
        let mut m = Matrix::from_vec(build_data());

        m.prepend_row(vec!("Grape", "Blue"));

        assert_eq!(
            vec!(
                vec!("Grape", "Blue"),
                vec!("Apple", "Red"),
                vec!("Pear", "Green"),
                vec!("Banana", "Yellow"),
                vec!("Orange", "Orange"),
            ),
            *m.data()
        );
        assert_eq!(5, m.rows);
    }
}
