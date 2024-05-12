pub fn reverse_it(v: i32) -> String {
    let res : i32;
    let mut signe : String = "".to_string();

    if v < 0 {
        signe = "-".to_string();
        res = v * (-1);
    } else {
        res = v;
    }

    let result : String = res.to_string().chars().rev().collect();
    format!("{}{}{}", signe, result, res)
}



fn main() {
    println!("{}", reverse_it(123));
    println!("{}", reverse_it(-123));
}