fn main() {
    let matrix = Matrix::new(&[&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]]);
    println!("{}", matrix);
}

// **************************************************
// **************************************************
// **************************************************
// **************************************************


pub struct Matrix(pub Vec<Vec<i32>>);

impl Matrix {
    pub fn new(slice: &[&[i32]]) -> Self {
        let mut matrix = Vec::new();
        for row in slice {
            matrix.push(row.to_vec());
        }
        Matrix(matrix)
    }
}

use std::fmt;

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, row) in self.0.iter().enumerate() {
            write!(f, "(")?;

            for (i, &elem) in row.iter().enumerate() {
                write!(f, "{}", elem)?;
                if i < row.len() - 1 {
                    write!(f, " ")?;
                }
            }
            
            write!(f, ")")?;
            if index < self.0.len() - 1 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}
