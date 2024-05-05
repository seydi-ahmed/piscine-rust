pub fn capitalize_first(input: &str) -> String {
    let mut res : String = String::new();
    let mut i : usize = 0;

    for lettre in input.chars(){
        if i == 0 {
            res.push_str(&lettre.to_uppercase().to_string());
            i += 1;
        } else {
            res.push_str(&lettre.to_string());
        }
        
    }

    res
}

pub fn title_case(input: &str) -> String {
    let mut res : String = String::new();

    for lettre in input.split_whitespace(){
        res.push_str(&capitalize_first(lettre));
        res.push_str(&" ".to_string());
    }

    res.trim().to_string()
}

pub fn change_case(input: &str) -> String {
    let mut res : String = String::new();

    for lettre in input.chars(){
        if lettre.is_uppercase(){
            res.push_str(&lettre.to_lowercase().to_string());
        } else if lettre.is_lowercase(){
            res.push_str(&lettre.to_uppercase().to_string());
        } else {
            res.push_str(&lettre.to_string());
        }
    }

    res
}



fn main() {
    println!("{}", capitalize_first("joe is missing"));
    println!("{}", title_case("jill is leaving A"));
    println!("{}",change_case("heLLo THere"));
}