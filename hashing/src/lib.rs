use std::collections::HashMap;


pub fn in_the_vec(words: Vec<i32>, word: i32) -> bool {
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

pub fn word_frequency_counter(words: Vec<i32>) -> HashMap<i32, usize> {
    let mut tab_deja_verfie : Vec<i32> = Vec::new();
    let mut hash_map : HashMap<i32, usize> = HashMap::new();
    
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

pub fn mean(list: &Vec<i32>) -> f64 {
    let mut sum = 0.0;
    let mut count = 0.0;

    for val in list {
        sum += f64::from(*val);
        count += 1.0;
    }

    sum / count
}


pub fn median(list: &Vec<i32>) -> i32 {
    let mut list1 = list.clone();
    list1.sort();
    
    if list1.len() % 2 == 1{
        list1[list1.len() / 2]
    } else{
        (list1[list1.len()/2] + list1[(list1.len() - 1)/2]) / 2
    }
}

pub fn mode(list: &Vec<i32>) -> i32 {
    let mut list1 = list.clone();
    list1.sort();
    let hmap = word_frequency_counter(list1);
    let mut key : usize = 0;
    let mut value : i32 = 0;

    for (k, v) in hmap{
        if v > key{
            value = k;
            key = v;
        }
    }

    value
}
