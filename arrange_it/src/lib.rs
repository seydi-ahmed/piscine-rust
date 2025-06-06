// pub fn arrange_phrase(phrase: &str) -> String {
//     let mut tab_phrase: Vec<String> = Vec::new();
//     let mut tab_phrase_mot: Vec<Vec<String>> = Vec::new();

//     for mot in phrase.split_whitespace() {
//         tab_phrase.push(mot.to_string());
//     }

//     for mot in tab_phrase {
//         let mut letter_by_letter: Vec<String> = Vec::new();
//         for letter in mot.chars() {
//             letter_by_letter.push(letter.to_string());
//         }
//         tab_phrase_mot.push(letter_by_letter);
//     }

//     let mut tab_chiffre : Vec<usize> = Vec::new();
    
//     for vecteur in tab_phrase_mot.clone() {
//         for lettre_in_vecteur in vecteur {
//             for lettre_in_vecteur_meme in lettre_in_vecteur.chars() {
//                 if lettre_in_vecteur_meme.is_numeric() {
//                     let n : usize = lettre_in_vecteur.parse().unwrap();
//                     tab_chiffre.push(n);
//                 }
//                 break;
//             }
//         }
//     }
    
//     let mut tab_phrase_mot_ordonne: Vec<Vec<String>> = tab_phrase_mot.clone();
//     let mut ind : usize = 0;
//     for pos in tab_chiffre.clone(){
//         tab_phrase_mot_ordonne[pos-1] = tab_phrase_mot[ind].clone();
//         ind += 1;
//     }

//     let mut result : String = String::new();
//     for vecteur in tab_phrase_mot_ordonne.clone(){
//         let vecteur1: String = vecteur.join("");
//         result.push_str(&vecteur1);
//         result.push_str(" ");
//     }

//     let result = result.replace("1", "");
//     let result = result.replace("2", "");
//     let result = result.replace("3", "");
//     let result = result.replace("4", "");
//     let result = result.replace("5", "");
//     let result = result.replace("6", "");
//     let result = result.replace("7", "");
//     let result = result.replace("8", "");
//     let result = result.replace("9", "");

//     let rst : String = result.trim().to_string();
//     rst

// }


pub fn arrange_phrase(phrase: &str) -> String {
    // Convert the string into a vector of words
    let mut words: Vec<&str> = phrase.split_whitespace().collect();

    // Sort the words based on the position number within each word
    words.sort_by_key(|word| {
        // Find the position number in the word
        let pos = word.chars().find(|c| c.is_numeric()).unwrap();
        pos.to_digit(10).unwrap() // Convert the digit character to a numeric value
    });

    // Join the sorted words into a single string
    let mut result = words.join(" ");
    // result

    for c in "123456789".chars() {
        result = result.replace(c, "");
    }

    let rst : String = result.trim().to_string();
    rst
}