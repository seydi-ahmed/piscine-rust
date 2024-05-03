// pub fn rot21(input: &str) -> String {
//     input.chars().map(|c| {
//         if c.is_ascii_alphabetic() {
//             let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
//             (((c as u8 - base + 21) % 26) + base) as char
//         } else {
//             c
//         }
//     }).collect()
// }


pub fn rot21(input: &str) -> String {
    let mut res = String::new();

    for c in input.chars() {
        if c >= 'a' && c <= 'z' {
            let rotated = rotate_lowercase(c, 21);
            res.push_str(&rotated.to_string());
        } else if c >= 'A' && c <= 'Z' {
            let rotated = rotate_uppercase(c, 21);
            res.push_str(&rotated.to_string());
        } else {
            res.push_str(&c.to_string());
        }
    }

    res
}

fn rotate_lowercase(c: char, shift: u8) -> char {
    let base = b'a';
    let rotated = ((c as u8 - base + shift) % 26) + base;
    rotated as char
}

fn rotate_uppercase(c: char, shift: u8) -> char {
    let base = b'A';
    let rotated = ((c as u8 - base + shift) % 26) + base;
    rotated as char
}


// ***********************************************************************
// ***********************************************************************
// ***********************************************************************
// ***********************************************************************

fn main() {
    println!("The letter \"a\" becomes: {}", rot21("a"));
    println!("The letter \"m\" becomes: {}", rot21("m"));
    println!("The word \"MISS\" becomes: {}", rot21("MISS"));
    println!("Your cypher wil be: {}", rot21("Testing numbers 1 2 3"));
    println!("Your cypher wil be: {}", rot21("rot21 works!"));
}
