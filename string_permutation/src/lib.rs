use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    let mut s1_vec: Vec<String> = Vec::new();
    let mut s2_vec: Vec<String> = Vec::new();

    for ss1 in s1.chars(){
        s1_vec.push(ss1.to_string());
    }
    for ss2 in s2.chars(){
        s2_vec.push(ss2.to_string());
    }

    let mut hash_map_s1: HashMap<&str, usize> = HashMap::new();
    let mut hash_map_s2: HashMap<&str, usize> = HashMap::new();

    
}

pub fn word_frequency_counter(words: Vec<&str>) -> HashMap<&str, usize> {
    let mut tab_deja_verfie: Vec<&str> = Vec::new();
    let mut hash_map: HashMap<&str, usize> = HashMap::new();

    for word in words.clone() {
        let mut nombre_de_fois: usize = 0;

        if in_the_vec(tab_deja_verfie.clone(), word) == false {
            for word_2 in words.clone() {
                if word_2 == word {
                    nombre_de_fois += 1;
                }
            }
            tab_deja_verfie.push(word);
            hash_map.insert(word, nombre_de_fois);
        }
    }

    hash_map
}

pub fn in_the_vec(words: Vec<&str>, word: &str) -> bool {
    let mut index: usize = 0;
    for mot in words {
        if mot == word {
            index += 1;
        }
    }
    if index != 0 {
        return true;
    }
    false
}
