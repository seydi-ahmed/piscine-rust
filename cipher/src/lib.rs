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
    if original.is_empty() || ciphered.is_empty() {
        return None;
    }


    if original.len() != ciphered.len() {
        return Some(Err(CipherError::new(false, ciphered.to_string())));
    }

    let mut result = true;
    for (orig_char, ciphered_char) in original.chars().zip(ciphered.chars()) {
        if atbash(orig_char) != ciphered_char {
            result = false;
            break;
        }
    }

    if result {
        Some(Ok(true))
    } else {
        Some(Err(CipherError::new(false, ciphered.to_string())))
    }
}

fn atbash(c: char) -> char {
    match c {
        'a'..='z' => ('z' as u8 - (c as u8 - 'a' as u8)) as char,
        'A'..='Z' => ('Z' as u8 - (c as u8 - 'A' as u8)) as char,
        _ => c,
    }
}

