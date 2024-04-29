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
    // Filtrer les caractères alphabétiques de la chaîne originale et chiffrée
    let original_filtered: String = original.chars().filter(|&c| c.is_alphabetic()).collect();
    let ciphered_filtered: String = ciphered.chars().filter(|&c| c.is_alphabetic()).collect();

    // Vérifier si les chaînes filtrées sont vides
    if original_filtered.is_empty() || ciphered_filtered.is_empty() {
        return None;
    }

    // Vérifier si les chaînes filtrées ont la même longueur
    if original_filtered.len() != ciphered_filtered.len() {
        return Some(Err(CipherError::new(false, generate_expected(original))));
    }

    // Appliquer le chiffrement à la chaîne d'origine et comparer avec la chaîne chiffrée fournie
    let result = original_filtered.chars().map(atbash).collect::<String>() == ciphered_filtered;

    if result {
        Some(Ok(true))
    } else {
        Some(Err(CipherError::new(false, generate_expected(original))))
    }
}


fn generate_expected(original: &str) -> String {
    original
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                atbash(c)
            } else {
                c
            }
        })
        .collect()
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