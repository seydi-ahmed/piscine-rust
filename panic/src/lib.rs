use std::fs::File;
use std::path::Path;

pub fn open_file(s: &str) -> File {
    // Tente d'ouvrir le fichier
    match File::open(Path::new(s)) {
        // Si l'ouverture réussit, renvoie le fichier
        Ok(file) => file,
        // Si le fichier n'existe pas, panique avec un message d'erreur
        Err(_) => panic!("File not found"),
    }
}
