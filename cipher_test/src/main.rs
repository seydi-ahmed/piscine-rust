#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CipherError {
    validation: bool,
    expected: String
}

impl CipherError {
    pub fn new(validation: bool, expected: String) -> CipherError {
        CipherError {
            validation,
            expected
        }
    }
}

pub fn cipher(original: &str, ciphered: &str) -> Option<Result<bool, CipherError>> {
    let original_filtered : String = original.chars().filter(|&c| c.is_alphabetic()).collect();
    let ciphered_filtered : String = ciphered.chars().filter(|&c| c.is_alphabetic()).collect();

    if original_filtered.is_empty() || ciphered_filtered.is_empty(){
        return None;
    }

    if original_filtered.len() != ciphered_filtered.len(){
        return Some(Err(CipherError::new(false, generated_expected(original))));
    }

    let result = original_filtered.chars().map(atbash).collect::<String>() == ciphered_filtered;
    if result {
        return Some(Ok(true));
    } else {
        return Some(Err(CipherError::new(false, generated_expected(original))));
    }

}

pub fn generated_expected(original: &str) -> String {
    original
        .chars()
        .map(|c| {
                if c.is_ascii_alphabetic() {
                    atbash(c)
                } else {
                    c
                }
            }
        )
        .collect()
}

pub fn atbash(c: char) -> char {
    if c >= 'a' && c <= 'z' {
        return ('z' as u8 - (c as u8 - 'a' as u8)) as char;
    } else if c >= 'A' && c <= 'Z' {
        return ('Z' as u8 - (c as u8 - 'A' as u8)) as char;
    } else {
        return c;
    }
}

fn main() {
    println!("{:?}", cipher("1Hello 2world!", "1Svool 2dliow!"));
    println!("{:?}", cipher("1Hello 2world!", "svool"));
    println!("{:?}", cipher("", "svool"));
}