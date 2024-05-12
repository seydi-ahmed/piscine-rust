pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (c, (c as f64).exp(), (c as f64).abs().ln())
}

pub fn str_function(a: String) -> (String, String) {
    let mut aa : String = String::new();

    for elem in a.split_whitespace() {
        let mut res : f64 = elem.parse().unwrap();
        res = res.exp();
        aa.push_str(&res.to_string());
        aa.push_str(&" ".to_string());
    }
    (a, aa.trim().to_string())
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let bb : Vec<f64> = Vec::new();

    for elem in b {
        let res = (elem as f64).abs().ln();
        bb.push(res);
    }

    (b, bb)
}



fn main() {
    let a: i32 = 0;
    let b = String::from("1 2 4 5 6");
    let c = vec![1, 2, 4, 5];

    println!("{:?}", nbr_function(a));
    println!("{:?}", str_function(b));
    println!("{:?}", vec_function(c));
}