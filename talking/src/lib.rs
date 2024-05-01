pub fn talking(text: &str) -> &str {
    // Vérifier si le texte est vide ou contient uniquement des espaces
    if text.trim().is_empty() {
        return "Just say something!";
    }

    // Vérifier si le texte est tout en majuscules
    let is_yelling = text.chars().all(|c| c.is_uppercase());

    // Vérifier si le texte se termine par un point d'interrogation
    let is_question = text.trim().ends_with('?');

    // Retourner la réponse appropriée en fonction des conditions
    match (is_yelling, is_question) {
        (true, true) => "Quiet, I am thinking!",
        (true, false) => "There is no need to yell, calm down!",
        (false, true) => "Sure.",
        _ => "Interesting",
    }
}
