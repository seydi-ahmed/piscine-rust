pub fn reverse_it(v: i32) -> String {
    let mut signe = "".to_string();
    let mut res = v;

    if v < 0 {
        signe = "-".to_string();
        res = res * (-1);
    }

    let result : String = res.to_string().chars().rev().collect();
    format!("{}{}{}", signe, result, res)
}


fn main() {
    println!("{}", reverse_it(123));
    println!("{}", reverse_it(-123));
}