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
        line.push_str(&" ".repeat(num_spaces)); // Ajout des espaces à la fin
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
        line.push_str(&" ".repeat(num_spaces)); // Ajout des espaces à la fin
        result.push(line);
    }

    result
}

