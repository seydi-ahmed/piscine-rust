pub fn capitalize_first(input: &str) -> String {
    let mut res : String = String::new();
    // let taille : usize = input.len()-1;

    for letter in input.chars(){
        res.push_str( &letter.to_string().to_uppercase());
    }

    let mut resultat : String = String::new();
    let mut index : usize = 0;
    for letter in res.chars(){
        if index != 0{
            resultat.push_str(&letter.to_string().to_lowercase());
            index += 1;
        }else{
            resultat.push_str(&letter.to_string());
            index += 1;
        }
    }

    resultat
}

pub fn title_case(input: &str) -> String {
    let res : Vec<&str> = input.split(' ').collect();
    let mut res1 : Vec<String> = Vec::new();

    for word in res{
        res1.push(capitalize_first(word));
    }

    let mot : String = res1.join(" ");
    mot
}

pub fn change_case(input: &str) -> String {
    let res : String = String::from(input);
    let mut final_word : String = String::new();

    for letter in res.chars(){
        if letter.is_uppercase(){
            let lettre : String = (letter.to_lowercase()).to_string();
            final_word.push_str(&lettre);
        } else if letter.is_lowercase(){
            let lettre : String = (letter.to_uppercase()).to_string();
            final_word.push_str(&lettre);
        } else{
            let lettre : String = letter.to_string();
            final_word.push_str(&lettre);
        }
    }

    final_word
}
