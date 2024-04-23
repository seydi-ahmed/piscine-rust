pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut my_vector: Vec<String> = Vec::new();

    for name in names{
        let (premier_mot, dernier_mot) = name.split_at(name.find(' ').unwrap());

        // let valeur_du_vecteur : String = premier_mot.get(0..1).unwrap().to + ". ".to_string() + dernier_mot.get(1..2).unwrap();
        let value = format!("{}. {}.",premier_mot.get(0..1).unwrap(), dernier_mot.trim().get(0..1).unwrap() );
        my_vector.push(value);
    }

    my_vector
}