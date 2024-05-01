pub fn score(word: &str) -> u64 {
    // Define the letter values
    let letter_values: [(char, u64); 26] = [
        ('A', 1), ('B', 3), ('C', 3), ('D', 2), ('E', 1), ('F', 4), ('G', 2), ('H', 4),
        ('I', 1), ('J', 8), ('K', 5), ('L', 1), ('M', 3), ('N', 1), ('O', 1), ('P', 3),
        ('Q', 10), ('R', 1), ('S', 1), ('T', 1), ('U', 1), ('V', 4), ('W', 4), ('X', 8),
        ('Y', 4), ('Z', 10)
    ];

    // Convert the word to uppercase to match the letter values
    let uppercase_word = word.to_uppercase();

    // Calculate the total score
    let mut total_score = 0;
    for letter in uppercase_word.chars() {
        // Check if the character is a letter and has a defined value
        if letter.is_ascii_alphabetic() {
            if let Some(&(_, value)) = letter_values.iter().find(|&&(c, _)| c == letter) {
                total_score += value;
            }
        }
    }

    total_score
}
