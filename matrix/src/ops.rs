#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

use matrix::*;


impl<T: Scalar + Clone + std::ops::Add<Output = T>> Add for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.0.len() != rhs.0.len() || self.0[0].len() != rhs.0[0].len() {
            return None;
        }

        let mut result = vec![vec![T::zero(); self.0[0].len()]; self.0.len()];

        for i in 0..self.0.len() {
            for j in 0..self.0[0].len() {
                result[i][j] = self.0[i][j].clone() + rhs.0[i][j].clone();
            }
        }

        Some(Matrix(result))
    }
}

impl<T: Scalar + Clone + std::ops::Sub<Output = T>> Sub for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.0.len() != rhs.0.len() || self.0[0].len() != rhs.0[0].len() {
            return None;
        }

        let mut result = vec![vec![T::zero(); self.0[0].len()]; self.0.len()];

        for i in 0..self.0.len() {
            for j in 0..self.0[0].len() {
                result[i][j] = self.0[i][j].clone() - rhs.0[i][j].clone();
            }
        }

        Some(Matrix(result))
    }
}
