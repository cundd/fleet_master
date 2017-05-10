use std::fmt::Debug;

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix<T> where T: Clone {
    pub rows: usize,
    pub columns: usize,
    pub data: Vec<Vec<T>>
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

    pub fn from_vec_ref(input: &Vec<Vec<T>>) -> Matrix<T> {
        Matrix::from_vec(input.to_owned())
    }

    pub fn from_slice(input: &[&[T]]) -> Matrix<T> {
        Matrix::from_vec(input.iter().map(|x| x.to_vec()).collect())
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
        assert_eq!(build_data(), m.data);
    }

    #[test]
    fn from_vec_ref_test() {
        let m = Matrix::from_vec_ref(&build_data());

        assert_eq!(4, m.rows);
        assert_eq!(2, m.columns);
        assert_eq!(build_data(), m.data);
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
}