pub fn pig_latin(text: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut chars = text.chars();
    
    if let Some(first_char) = chars.next() {
        if vowels.contains(&first_char) {
            // If the word starts with a vowel, just add "ay" to the end.
            return format!("{}ay", text);
        } else {
            // Find the position of the first vowel.
            let first_vowel_index = text.find(|c: char| vowels.contains(&c)).unwrap_or(text.len());
            
            if text[first_vowel_index..].starts_with("qu") {
                // If the word starts with a consonant followed by "qu", move "qu" to the end.
                let prefix = &text[..first_vowel_index + 1];
                let suffix = &text[first_vowel_index + 1..];
                return format!("{}{}ay", suffix, prefix);
            } else {
                // If it starts with a consonant, move consonants before the first vowel to the end.
                let prefix = &text[..first_vowel_index];
                let suffix = &text[first_vowel_index..];
                return format!("{}{}ay", suffix, prefix);
            }
        }
    }
    
    // If the input is empty, return it as is.
    text.to_string()
}