pub fn rot21(input: &str) -> String {
    let mut res : String = String::new();

    for letter in input.chars(){
        if letter >= 'a' && letter <= 'z'{
            let letter_to_pushed = rotate_lowercase(letter, 21);
            res.push_str(&letter_to_pushed.to_string());
        } else if letter >= 'A' && letter <= 'Z'{
            let letter_to_pushed = rotate_uppercase(letter, 21);
            res.push_str(&letter_to_pushed.to_string());
        } else {
            res.push_str(&letter.to_string());
        }
    }

    res
}

pub fn rotate_lowercase(c: char, shift: u8) -> char {
    let base = b'a';
    let c_rotated = ((c as u8 - base + shift) % 26) + base;
    c_rotated as char
}

pub fn rotate_uppercase(c: char, shift: u8) -> char {
    let base = b'A';
    let c_rotated = ((c as u8 - base + shift) % 26) + base;
    c_rotated as char
}

// ***************************************************************************

fn main() {
    println!("The letter \"a\" becomes: {}", rot21("a"));
    println!("The letter \"m\" becomes: {}", rot21("m"));
    println!("The word \"MISS\" becomes: {}", rot21("MISS"));
    println!("Your cypher wil be: {}", rot21("Testing numbers 1 2 3"));
    println!("Your cypher wil be: {}", rot21("rot21 works!"));
}
