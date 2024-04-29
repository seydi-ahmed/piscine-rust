#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CipherError {
    pub validation: bool,
    pub expected: String,
}

impl CipherError {
    pub fn new(validation: bool, expected: String) -> CipherError {
        CipherError {
            validation,
            expected,
        }
    }
}

pub fn cipher(original: &str, ciphered: &str) -> Option<Result<bool, CipherError>> {
    let original_filtered: String = original.chars().filter(|&c| c.is_alphabetic()).collect();
    let ciphered_filtered: String = ciphered.chars().filter(|&c| c.is_alphabetic()).collect();

    if original_filtered.is_empty() || ciphered_filtered.is_empty() {
        return None;
    }

    if original_filtered.len() != ciphered_filtered.len() {
        return Some(Err(CipherError::new(false, generate_expected(&original, &ciphered))));
    }

    let mut result = true;
    let mut expected = String::new();
    for (orig_char, ciphered_char) in original.chars().zip(ciphered.chars()) {
        if orig_char.is_ascii_alphabetic() {
            expected.push(atbash(orig_char));
        } else {
            expected.push(orig_char);
        }
        if atbash(orig_char) != ciphered_char {
            result = false;
            break;
        }
    }

    if result {
        Some(Ok(true))
    } else {
        Some(Err(CipherError::new(false, expected)))
    }
}

fn generate_expected(original: &str, ciphered: &str) -> String {
    let mut expected = String::new();
    let mut ciphered_chars = ciphered.chars();
    for orig_char in original.chars() {
        if orig_char.is_ascii_alphabetic() {
            if let Some(_ciphered_char) = ciphered_chars.next() {
                expected.push(atbash(orig_char));
            } else {
                break;
            }
        } else {
            expected.push(orig_char);
        }
    }
    expected
}




pub fn atbash(c: char) -> char {
    match c {
        'a'..='z' => ('z' as u8 - (c as u8 - 'a' as u8)) as char,
        'A'..='Z' => ('Z' as u8 - (c as u8 - 'A' as u8)) as char,
        _ => c,
    }
}

// Some(Ok(true))
// Some(Err(CipherError { validation: false, expected: "1Svool 2dliow!" }))
// None