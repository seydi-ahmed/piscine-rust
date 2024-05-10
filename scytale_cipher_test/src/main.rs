fn main() {
    println!("\"scytale Code\" size=6 -> {:?}", scytale_cipher(String::from("scytale Code"), 6));
    println!("\"scytale Code\" size=8 -> {:?}", scytale_cipher(String::from("scytale Code"), 8));
}


fn scytale_cipher(message: String, i: u32) -> String {
    let mut result : String = String::new();

    if message.is_empty() || i == 0 {
        return result;
    }

    let chars_per_line : usize = (message.len() as f64 / i as f64).ceil() as usize;

    let mut matrix : Vec<Vec<char>> = vec![vec![' '; chars_per_line]; i as usize];

    for (index, c) in message.chars().enumerate() {
        let row = index % i as usize;
        let col = index / i as usize;
        matrix[row][col] = c;
    }

    for row in matrix {
        result.extend(row);
    }

    let res : String = result.trim().to_string();
    res
}