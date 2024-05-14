// // use core::ops::{Add , Sub , Mul};
// pub mod ops;
// use lalgebra_scalar::Scalar;

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct Matrix<T>(pub Vec<Vec<T>>);

// impl<T: Scalar + Clone> Matrix<T> {
//     pub fn new() -> Matrix<T> {
//         Matrix(vec![vec![T::zero(); 1]])
//     }

//     pub fn zero(row: usize, col: usize) -> Matrix<T> {
//         Matrix(vec![vec![T::zero(); col]; row])
//     }

//     pub fn identity(n: usize) -> Matrix<T> {
//         let mut matrix = Matrix::zero(n, n);
//         for i in 0..n {
//             matrix.0[i][i] = T::one();
//         }
//         matrix
//     }
// }


pub mod mult;
use lalgebra_scalar::Scalar;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar + Clone> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Matrix(vec![vec![T::zero(); 1]])
    }

    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        Matrix(vec![vec![T::zero(); col]; row])
    }

    pub fn identity(n: usize) -> Matrix<T> {
        let mut matrix = Matrix::zero(n, n);
        for i in 0..n {
            matrix.0[i][i] = T::one();
        }
        matrix
    }

    pub fn number_of_cols(&self) -> usize {
        if self.0.is_empty() {
            0
        } else {
            self.0[0].len()
        }
    }

    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        if n < self.number_of_rows() {
            self.0[n].clone()
        } else {
            Vec::new()
        }
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        if self.number_of_cols() == 0 {
            return Vec::new();
        }
        self.0.iter().map(|row| row[n].clone()).collect()
    }
}
