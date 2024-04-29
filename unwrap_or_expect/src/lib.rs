pub enum Security {
    Unknown,
    High,
    Medium,
    Low,
    BlockServer,
}

pub fn fetch_data(server: Result<String, String>, security_level: Security) -> String {
    // Commence par évaluer le niveau de sécurité
    match security_level {
        // Si le niveau de sécurité est "Unknown"
        Security::Unknown => {
            // Déballe la valeur de Result et renvoie la chaîne si Ok, ou panique sinon
            server.unwrap()
        }
        // Si le niveau de sécurité est "High"
        Security::High => {
            // Vérifie si Result est une erreur, panique avec un message personnalisé sinon déballe et renvoie la valeur
            if let Err(_err) = server {
                panic!("ERROR: program stops");
            } else {
                server.unwrap()
            }
        }
        // Si le niveau de sécurité est "Medium"
        Security::Medium => {
            // Vérifie si Result est une erreur, renvoie un avertissement sinon déballe et renvoie la valeur
            if server.is_err() {
                "WARNING: check the server".to_string()
            } else {
                server.unwrap()
            }
        }
        // Si le niveau de sécurité est "Low"
        Security::Low => {
            // Déballe la valeur de Result et renvoie la chaîne si Ok, ou formatte un message d'erreur sinon
            server.unwrap_or_else(|err| format!("Not found: {}", err))
        }
        // Si le niveau de sécurité est "BlockServer"
        Security::BlockServer => {
            // Vérifie si Result est une valeur Ok, panique avec l'URL du serveur sinon panique avec le message d'erreur
            if let Ok(server_url) = &server {
                panic!("{}", server_url);
            } else {
                server.expect("") // Panique avec le message d'erreur de Result
            }
        }
    }
}


