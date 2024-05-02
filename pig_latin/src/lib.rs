pub fn pig_latin(text: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    let mut chars = text.chars().peekable();
    let mut result = String::new();

    // ******************************
    let mut trouve : bool = false;
    // Move consonants before the first vowel to the end and add "ay"
    while let Some(&c) = chars.peek() {
        if vowels.contains(&c) {
            break;
        }
        trouve = true;
        result.push(c);
        chars.next(); // consume consonant
    }
    if trouve {
        let rest_of_word: String = chars.collect();
        result.push_str(&rest_of_word);
        result.push_str("ay");
        return result;
    }

    // ******************************

    // Check if the word starts with "qu"
    if text.starts_with("qu") {
        result.push_str(&text[2..]); // Skip the 'q' and 'u'
        result.push_str("quay");
        return result;
    }

    // Check if the word starts with a consonant followed by "qu"
    let mut consonants_before_qu = String::new();
    while let Some(&c) = chars.peek() {
        if !vowels.contains(&c) && c != 'q' {
            consonants_before_qu.push(c);
            chars.next(); // consume consonant
        } else {
            break;
        }
    }

    if let Some('q') = chars.peek() {
        chars.next(); // consume 'q'
        if let Some('u') = chars.peek() {
            chars.next(); // consume 'u'
            let rest_of_word: String = chars.collect();
            result.push_str(&rest_of_word);
            result.push_str(&consonants_before_qu);
            result.push_str("quay");
            return result;
        }
    }

    // If the first character is a vowel, just add "ay" to the end
    if let Some(&c) = chars.peek() {
        if vowels.contains(&c) {
            result.push_str(text);
            result.push_str("ay");
            return result;
        }
    }

    // Move consonants before the first vowel to the end and add "ay"
    let mut consonants = String::new();
    while let Some(&c) = chars.peek() {
        if vowels.contains(&c) {
            break;
        }
        consonants.push(c);
        chars.next(); // consume consonant
    }
    let rest_of_word: String = chars.collect();
    result.push_str(&rest_of_word);
    result.push_str(&consonants);
    result.push_str("ay");

    result
}
