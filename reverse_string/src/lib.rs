pub fn rev_str(input: &str) -> String {
    // let mut mot_rev = String::new();
    // for letter in input.chars().rev() {
    //     mot_rev += &letter.to_string();
    // }
    // mot_rev.to_string()

    input.chars().rev().collect()
}