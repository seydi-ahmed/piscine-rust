pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (c, (c as f64).exp(), (c as f64).ln())
}

pub fn str_function(a: String) -> (String, String) {
    let mut second = String::new();

    for number in a.split_whitespace() {
        let number_i: f64 = number.parse().unwrap();
        let number_i_exp = number_i.exp();

        second.push_str(&number_i_exp.to_string());
        second.push_str(" ");
    }

    (a, String::from(second.trim()))
}

// pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
// }
