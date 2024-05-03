pub fn nbr_function(c: i32) -> (i32, f64, f64) {
}

pub fn str_function(a: String) -> (String, String) {
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
}

fn main() {
    let a: i32 = 0;
    let b = String::from("1 2 4 5 6");
    let c = vec![1, 2, 4, 5];

    println!("{:?}", nbr_function(a));
    println!("{:?}", str_function(b));
    println!("{:?}", vec_function(c));
}