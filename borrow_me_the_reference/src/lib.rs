pub fn delete_function(v: Vec<String>, index: usize) -> Vec<String> {
    let mut index2: usize = 0;
    let mut v2: Vec<String> = Vec::new();

    for letter in v {
        if index != index2 {
            v2.push(letter);
        }
        index2 += 1;
    }
    v2
}

pub fn string_to_vec(s: String) -> Vec<String> {
    let mut s1: Vec<String> = Vec::new();

    for letter in s.chars() {
        s1.push(letter.to_string());
    }
    s1
}

pub fn delete_and_backspace(s: &mut String) -> Vec<String> {

    let mut tab_vector : Vec<String> = string_to_vec(s.to_string());
    let _taille : usize = tab_vector.len()-2;

    let mut trouve : bool = false;

    for mut i in 0.._taille-2{
        if trouve{
            i = 0;
        }
        if tab_vector[i] == "-"{
            tab_vector = delete_function(tab_vector, i-1);
            tab_vector = delete_function(tab_vector, i);
            trouve = true;
        } else {
            trouve = false;
        }
    }

    tab_vector
}

// pub fn do_operations(v: &mut Vec<String>) {
// }

