// *************************************

pub fn reverse_it(v: i32) -> String {
    let mut v1: i32 = v.clone();
    let mut new_str: String = String::new();

    if v * (-1) >= 0 {
        v1 = -v1;
        new_str.push_str(&"-".to_string());
    }

    let mut tab: Vec<i32> = Vec::new();

    while v1 > 0 {
        let q: i32 = v1 / 10;
        let r: i32 = v1 % 10;
        tab.push(r);
        v1 = q;
    }

    for nombre in tab.clone() {
        new_str.push_str(&nombre.to_string());
    }

    for nombre in tab.iter().rev() {
        new_str.push_str(&nombre.to_string());
    }

    new_str
}

// *************************************

use std::collections::HashMap;

pub fn counting_words(words: &str) -> HashMap<String, u32> {
    let mut word_count: HashMap<String, u32> = HashMap::new();

    // Remove punctuation except for apostrophes
    let words = words.replace(|c: char| !c.is_alphanumeric() && c != '\'', " ");

    // Split the string into words by whitespace
    for word in words.split_whitespace() {
        // Convert the word to lowercase
        let word = word.to_lowercase();
        // Increment the count for this word in the HashMap
        *word_count.entry(word).or_insert(0) += 1;
    }

    word_count
}

// *************************************
