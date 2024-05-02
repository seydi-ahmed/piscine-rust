pub fn get_diamond(c: char) -> Vec<String> {
    let mut result = Vec::new();

    // Convertir la lettre en caractère ASCII et obtenir la distance par rapport à 'A'
    let distance = c as u8 - b'A';

    // Calculer la largeur du diamant
    let _width = (distance * 2 + 1) as usize;

    // Construire la moitié supérieure du diamant
    for i in 0..=distance {
        let mut line = String::new();
        let num_spaces = distance as usize - i as usize;
        line.push_str(&" ".repeat(num_spaces));
        line.push((b'A' + i) as char);
        if i > 0 {
            let num_middle_spaces = (i * 2 - 1) as usize;
            line.push_str(&" ".repeat(num_middle_spaces));
            line.push((b'A' + i) as char);
        }
        result.push(line);
    }

    // Construire la moitié inférieure du diamant
    for i in (0..distance).rev() {
        let mut line = String::new();
        let num_spaces = distance as usize - i as usize;
        line.push_str(&" ".repeat(num_spaces));
        line.push((b'A' + i) as char);
        if i > 0 {
            let num_middle_spaces = (i * 2 - 1) as usize;
            line.push_str(&" ".repeat(num_middle_spaces));
            line.push((b'A' + i) as char);
        }
        result.push(line);
    }

    // Si le diamant ne contient qu'une seule ligne, retourner directement le résultat
    if result.len() == 1 {
        return result;
    }

    // Copier la moitié supérieure du diamant (à l'exception de la première ligne) pour former la moitié inférieure
    let mut lower_half = result[1..].iter().rev().cloned().collect::<Vec<_>>();

    // Ajouter la moitié inférieure à la moitié supérieure
    result.append(&mut lower_half);

    result
}
