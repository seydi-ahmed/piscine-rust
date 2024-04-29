pub struct Message {
    content: String,
    user: String,
}

impl Message {
    pub fn new(ms: String, u: String) -> Message {
        Message {
            content: ms,
            user: u,
        }
    }

    pub fn send_ms(&self) -> Option<&str> {
        // Vérifie si le contenu du message est vide ou contient le mot "stupid"
        if self.content.is_empty() || self.content.contains("stupid") {
            None
        } else {
            Some(&self.content)
        }
    }
}

pub fn check_ms(ms: &Message) -> (bool, &str) {
    // Appelle la méthode send_ms de Message pour vérifier le contenu du message
    match ms.send_ms() {
        // Si send_ms retourne None, renvoie (false, "ERROR: illegal")
        None => (false, "ERROR: illegal"),
        // Sinon, renvoie (true, contenu du message)
        Some(content) => (true, content),
    }
}
