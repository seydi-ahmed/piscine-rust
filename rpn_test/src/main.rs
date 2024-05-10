fn main() {
    let args: Vec<String> = std::env::args().collect();

    rpn(&args[1]);
}

pub fn rpn(expr: &str) {
    let mut stack : Vec<i64> = Vec::new();

    
}