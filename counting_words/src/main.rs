use std::collections::HashMap;

fn counting_words(words: &str) -> HashMap<String, u32> {
    let mut result : HashMap<String, u32> = HashMap::new();

    let words = words.replace(|c: char| !c.is_alphanumeric() && c != '\'', " ");

    for mot in words.split_whitespace() {
        let mut mot = mot.to_lowercase();

        if mot.starts_with('\'') && mot.ends_with('\'') {
            mot.remove(0);
            mot.remove(mot.len() - 1);
        }

        if mot != ""{
            *result.entry(mot).or_insert(0) += 1;
        }
    }

    result
}


fn main() {
    println!("{:?}", counting_words("Hello, world!"));
    println!("{:?}", counting_words("“Two things are infinite: the universe and human stupidity; and I'm not sure about the universe.”
    ― Albert Einstein "));
    println!("{:?}", counting_words("Batman, BATMAN, batman, Stop stop"));
    println!("{:?}", counting_words("Batman, BATMAN, 'batman', Stop stop"));
}