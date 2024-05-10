fn main() {
    let args: Vec<String> = std::env::args().collect();

    rpn(&args[1]);
}


pub fn rpn(expr: &str) {
    let mut result : Vec<i64> = Vec::new();

    for token in expr.split_whitespace() {
        match token {
            "+" => {
                if let (Some(b), Some(a)) = (result.pop(), result.pop()){
                    result.push(a + b);
                } else {
                    println!("Error");
                    return;
                }
            }
            "-" => {
                if let (Some(b), Some(a)) = (result.pop(), result.pop()){
                    result.push(a - b);
                } else {
                    println!("Error");
                    return;
                }
            }
            "*" => {
                if let (Some(b), Some(a)) = (result.pop(), result.pop()){
                    result.push(a * b);
                } else {
                    println!("Error");
                    return;
                }
            }
            "/" => {
                if let (Some(b), Some(a)) = (result.pop(), result.pop()){
                    if b != 0 {
                        result.push(a / b);
                    } else {
                        println!("Error");
                        return;
                    }
                } else {
                    println!("Error");
                    return;
                }
            }
            "%" => {
                if let (Some(b), Some(a)) = (result.pop(), result.pop()){
                    if b != 0 {
                        result.push(a % b);
                    } else {
                        println!("Error");
                        return;
                    }
                } else {
                    println!("Error");
                    return;
                }
            }
            _ => {
                if let Ok(num) = token.parse::<i64>() {
                    result.push(num);
                } else {
                    println!("Error");
                    return;
                }
            }
        }
    }

    if result.len() == 1 {
        println!("{}", result.pop().unwrap());
    } else {
        println!("Error");
    }
}