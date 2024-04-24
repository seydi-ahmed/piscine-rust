pub fn arrange_phrase(phrase: &str) -> Vec<Vec<String>> {
    // let mut result : String = String::new();
    let mut tab_phrase : Vec<String> = Vec::new();
    let mut tab_phrase_mot : Vec<Vec<String>> = Vec::new();

    for mot in phrase.split_whitespace(){
        tab_phrase.push(mot.to_string());
    }

    for mot in tab_phrase{
        let mut letter_by_letter : Vec<String> = Vec::new();
        for letter in mot.chars(){
            letter_by_letter.push(letter.to_string());
        }
        tab_phrase_mot.push(letter_by_letter);
    }

    let mut tab_phrase_mot_ordonne : Vec<Vec<String>> = Vec::new();
    for mut vecteur in tab_phrase_mot{
        let mut position_de_i : usize = 0;
        for lettre_in_vecteur in vecteur.clone(){
            for lettre_in_vecteur_meme in lettre_in_vecteur.chars(){
                if lettre_in_vecteur_meme.is_numeric(){
                    vecteur = amener_chiffre_devant(vecteur, 0, position_de_i);break;
                }
                break;
            }
            position_de_i += 1;
        }
        
        tab_phrase_mot_ordonne.push(vecteur);
    }

    tab_phrase_mot_ordonne
}

pub fn amener_chiffre_devant(mut tab_vecteur: Vec<String>, index1: usize, index2: usize) -> Vec<String>{
    let temps = tab_vecteur[index1].clone();
    tab_vecteur[index1] = tab_vecteur[index2].clone();
    tab_vecteur[index2] = temps;

    tab_vecteur
}