pub fn matrix_determinant(matrix: [[isize; 3]; 3]) -> isize {
    let matrix_a : isize = matrix[0][0];
    let matrix_b : isize = matrix[0][1];
    let matrix_c : isize = matrix[0][2];

    let matrix_aa : isize = (matrix[1][1]*matrix[2][2])-(matrix[1][2]*matrix[2][1]);
    let matrix_bb : isize = (matrix[1][0]*matrix[2][2])-(matrix[1][2]*matrix[2][0]);
    let matrix_cc : isize = (matrix[1][0]*matrix[2][1])-(matrix[1][1]*matrix[2][0]);

    let res_a : isize = matrix_a * matrix_aa;
    let res_b : isize = matrix_b * matrix_bb;
    let res_c : isize = matrix_c * matrix_cc;

    return res_a-res_b+res_c;
}


fn main() {
    let matrix = [[1, 2, 4], [2, -1, 3], [4, 0, 1]];

    println!(
        "The determinant of the matrix:\n|1  2  4|\n|2 -1  3|  = {}\n|4  0  1|",
        matrix_determinant(matrix)
    );
}