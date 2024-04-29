pub enum Security {
    Unknown,
    High,
    Medium,
    Low,
    BlockServer,
}

pub fn fetch_data(server: Result<String, String>, security_level: Security) -> String {
    match security_level {
        Security::Unknown => server.expect(""),
        Security::High => {
            if let Err(err) = server {
                panic!("ERROR: {}", err);
            } else {
                server.unwrap()
            }
        }
        Security::Medium => {
            if server.is_err() {
                "WARNING: check the server".to_string()
            } else {
                server.unwrap()
            }
        }
        Security::Low => server.unwrap_or_else(|err| format!("Not found: {}", err)),
        Security::BlockServer => server.expect(""),
    }
}
