pub fn scytale_cipher(message: String, i: u32) -> String {
    let mut result = String::new();

    // Si la taille du message est 0 ou si le nombre de tours est 0, retourne une chaîne vide
    if message.is_empty() || i == 0 {
        return result;
    }

    // Calcul du nombre de caractères par ligne
    let chars_per_line = (message.len() as f64 / i as f64).ceil() as usize;

    // Création d'une matrice de caractères pour stocker les caractères dans l'ordre de la scytale
    let mut matrix: Vec<Vec<char>> = vec![vec![' '; chars_per_line]; i as usize];

    // Remplissage de la matrice avec les caractères du message
    for (index, c) in message.chars().enumerate() {
        let row = index % i as usize;
        let col = index / i as usize;
        matrix[row][col] = c;
    }

    // Construction de la chaîne résultante à partir de la matrice
    for row in matrix {
        result.extend(row);
    }

    // Retirer le dernier espace s'il existe
    if let Some(' ') = result.chars().last() {
        result.pop();
    }

    result
}