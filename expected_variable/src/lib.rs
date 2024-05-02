use edit_distance::levenshtein;

pub fn expected_variable(source: &str, target: &str) -> Option<String> {

    if source.contains(" ") || source.contains("-") || source.chars().all(|ch| ch.is_ascii_lowercase()) || source.chars().all(|ch| ch.is_ascii_uppercase()){
        return  None;
    }
    let res = levenshtein(source.to_lowercase().as_str(), target.to_lowercase().as_str());
    
    println!("res {}", res);
    if res < target.len() / 2{ 
        let percentage = (100 - (res as f64 / target.len() as f64 * 100.0) as u8) as u8;
        
        return Some(format!("{}%", percentage));
    }
    None
}