use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    let mut s1_vec: Vec<char> = Vec::new();
    let mut s2_vec: Vec<char> = Vec::new();

    for ss1 in s1.chars(){
        s1_vec.push(ss1);
    }
    for ss2 in s2.chars(){
        s2_vec.push(ss2);
    }

    let hash_map_s1: HashMap<char, usize> = HashMap::from(word_frequency_counter(s1_vec));
    let hash_map_s2: HashMap<char, usize> = HashMap::from(word_frequency_counter(s2_vec));

    // hash_map_s1 = word_frequency_counter(s1_vec);
    // hash_map_s2 = word_frequency_counter(s2_vec);

    hash_map_s1 == hash_map_s2
}

pub fn word_frequency_counter(words: Vec<char>) -> HashMap<char, usize> {
    let mut tab_deja_verfie: Vec<char> = Vec::new();
    let mut hash_map: HashMap<char, usize> = HashMap::new();

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

pub fn in_the_vec(words: Vec<char>, word: char) -> bool {
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
