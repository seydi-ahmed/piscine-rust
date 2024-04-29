use std::fs::OpenOptions;
use std::io::Write;

pub fn open_or_create(file: &str, content: &str) {
    // Ouvre le fichier avec l'option de création s'il n'existe pas
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(file)
        .expect("Unable to open or create file");

    // Écrit le contenu dans le fichier
    file.write_all(content.as_bytes())
        .expect("Unable to write to file");
}
