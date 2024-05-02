pub fn rotate(input: &str, key: i8) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                let rotated = (((c as i8 - base as i8 + key) % 26 + 26) % 26 + base as i8) as u8;
                rotated as char
            } else {
                c
            }
        })
        .collect()
}