pub fn stars(n: u32) -> String {
    // Calculate the number of stars: 2^n
    let num_stars = 2u32.pow(n);

    // Create a string containing the desired number of asterisks
    let mut result = String::new();
    for _ in 0..num_stars {
        result.push('*');
    }

    result // Return the resulting string
}
