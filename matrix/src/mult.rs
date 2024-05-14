use crate::Matrix;
use std::ops::Mul;
use lalgebra_scalar::Scalar;

impl<T: Mul<Output = T> + Clone + Scalar> Mul for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.number_of_cols() != rhs.number_of_rows() {
            return None;
        }

        let mut result = vec![vec![T::zero(); rhs.number_of_cols()]; self.number_of_rows()];

        for i in 0..self.number_of_rows() {
            for j in 0..rhs.number_of_cols() {
                for k in 0..self.number_of_cols() {
                    result[i][j] = result[i][j].clone() + self.0[i][k].clone() * rhs.0[k][j].clone();
                }
            }
        }

        Some(Matrix(result))
    }
}
