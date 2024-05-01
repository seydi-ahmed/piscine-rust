pub fn is_pangram(s: &str) -> bool {
    // Create a boolean array to track the presence of each letter
    let mut alphabet_present = [false; 26];

    // Iterate through each character in the string
    for c in s.chars() {
        // Convert the character to lowercase for case-insensitive comparison
        let lowercase_c = c.to_ascii_lowercase();
        // Check if the character is an ASCII letter and update the corresponding entry in the array
        if lowercase_c.is_ascii_alphabetic() {
            let index = (lowercase_c as u8 - b'a') as usize;
            alphabet_present[index] = true;
        }
    }

    // Check if all letters of the alphabet are present
    alphabet_present.iter().all(|&x| x)
}
