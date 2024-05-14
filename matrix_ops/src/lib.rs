// use core::ops::{Add , Sub , Mul};
pub mod ops;
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
}
