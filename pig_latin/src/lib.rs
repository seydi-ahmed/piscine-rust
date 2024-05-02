pub fn pig_latin(text: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];  // Array of vowels
  
    let mut translated = String::new();
    let mut first_consonants = String::new();

    // Si le texte commence par une consonne suivie de "qu", déplace-les à la fin et ajoute "ay"
    if text.starts_with(|c: char| !vowels.contains(&c)) && text.chars().nth(1) == Some('q') && text.chars().nth(2) == Some('u') {
        let mut chars = text.chars();
        first_consonants.push(chars.next().unwrap()); // Ajoute la première consonne
        first_consonants.push(chars.next().unwrap()); // Ajoute la lettre "u" après "q"
        translated.push_str(&chars.collect::<String>()); // Ajoute le reste du mot
        translated.push_str(&first_consonants); // Ajoute la consonne et "qu" à la fin
        translated.push_str("uay");  // Ajoute "ay" à la fin
        translated.remove(0);
return translated;

    }

    for c in text.chars() {
        if vowels.contains(&c) {
            translated.push_str(text);
            translated.push_str("ay");  // Add "ay" for words starting with vowels
            break;
        } else {
            first_consonants.push(c);
            if c == 'q' && text.chars().nth(1).unwrap_or(' ') == 'u' {
                first_consonants.push('u');  // Handle "qu" case
                break;
            }
        }
    }
  
    if !first_consonants.is_empty() {  // If consonants found, move them and add "ay"
        translated = text[first_consonants.len()..].to_string();
        translated.push_str(&first_consonants);
        translated.push_str("ay");
    }
  
    translated
}
