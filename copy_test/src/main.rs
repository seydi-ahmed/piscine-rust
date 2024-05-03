pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (c, (c as f64).exp(), (c as f64).abs().ln())
}

pub fn str_function(a: String) -> (String, String) {
    let mut result : String = String::new();

    for word in a.split_whitespace() {
        let word1 : f64 = word.parse().unwrap();
        let word2 = word1.exp();

        result.push_str(&word2.to_string());
        result.push_str(&" ".to_string());
    }

    (a, String::from(result.trim()))
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut res : Vec<f64> = Vec::new();

    for vec in b.clone() {
        res.push((vec as f64).ln());
    }

    (b,res)
}


fn main() {
    let a: i32 = 0;
    let b = String::from("1 2 4 5 6");
    let c = vec![1, 2, 4, 5];

    println!("{:?}", nbr_function(a));
    println!("{:?}", str_function(b));
    println!("{:?}", vec_function(c));
}