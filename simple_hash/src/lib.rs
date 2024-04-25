use std::collections::HashMap;


pub fn in_the_vec(words: Vec<&str>, word: &str) -> bool {
    let mut index : usize = 0;
    for mot in words{
        if mot == word{
            index += 1;
        }
    }
    if index != 0{
        return true;
    }
    false
}

pub fn word_frequency_counter(words: Vec<&str>) -> HashMap<&str, usize> {
    let mut tab_deja_verfie : Vec<&str> = Vec::new();
    let mut hash_map : HashMap<&str, usize> = HashMap::new();
    
    for word in words.clone(){
        let mut nombre_de_fois : usize = 0;

        if in_the_vec(tab_deja_verfie.clone(), word) == false{
            for word_2 in words.clone(){
                if word_2 == word{
                    nombre_de_fois += 1;
                }   
            }
            tab_deja_verfie.push(word);
            hash_map.insert(word, nombre_de_fois);
        }
    }

    hash_map
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    let mut taille : usize = 0;
    for (_, _) in frequency_count{
        taille += 1;
    }
    taille
}