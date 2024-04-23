pub fn first_subword(s: String) -> String {

    let underscores_par_espace = s.replace('_', " ");

    let mut result = String::new();
    let mut first_char = true;
    
    for letter in underscores_par_espace.chars() {
        if first_char {
            result.push(letter);
            first_char = false;
        } else if letter.is_ascii_uppercase() {
            result.push(' ');
        } else {
            result.push(letter);
        }
    }

    result
        .split_whitespace()
        .next()
        .unwrap_or_default()
        .to_string()
}