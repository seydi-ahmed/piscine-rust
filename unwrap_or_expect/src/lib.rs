pub enum Security {
    Unknown,
    High,
    Medium,
    Low,
    BlockServer,
}

pub fn fetch_data(server: Result<String, String>, security_level: Security) -> String {
    match security_level {
        Security::Unknown => server.unwrap(),
        Security::High => {
            if let Err(_err) = server {
                panic!("ERROR: program stops");
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
        Security::BlockServer => {
            if let Ok(server_url) = &server {
                panic!("{}", server_url);
            } else {
                server.expect("")
            }
        }
    }
}

