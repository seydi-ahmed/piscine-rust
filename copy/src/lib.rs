pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (c, (c as f64).exp(), (c as f64).ln())
}

pub fn str_function(a: String) -> (String, String) {
    let second = a.parse()::<f64>
}

// pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
// }