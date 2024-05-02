pub fn pig_latin(text: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];  // Array of vowels
  
    let mut translated = String::new();
    let mut first_consonants = String::new();
  
    for c in text.chars() {
      if vowels.contains(&c) {
        translated.push_str(text);
        translated.push_str("ay");  // Add "hay" for words starting with vowels
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
    //   translated.push_str("-");
      translated.push_str(&first_consonants);
      translated.push_str("ay");
    }
  
    translated
  }
  