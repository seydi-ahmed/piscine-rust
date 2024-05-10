pub fn insertion_sort(slice: &mut [i32], steps: usize) {
    (slice.split_at_mut(steps+1).0).sort()
}

// *************************************

pub fn rpn(expr: &str) {
    let mut stack: Vec<i64> = Vec::new();

    for token in expr.split_whitespace() {
        match token {
            "+" => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    stack.push(a + b);
                } else {
                    println!("Error");
                    return;
                }
            }
            "-" => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    stack.push(a - b);
                } else {
                    println!("Error");
                    return;
                }
            }
            "*" => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    stack.push(a * b);
                } else {
                    println!("Error");
                    return;
                }
            }
            "/" => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    if b == 0 {
                        println!("Error");
                        return;
                    }
                    stack.push(a / b);
                } else {
                    println!("Error");
                    return;
                }
            }
            "%" => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    if b == 0 {
                        println!("Error");
                        return;
                    }
                    stack.push(a % b);
                } else {
                    println!("Error");
                    return;
                }
            }
            _ => {
                if let Ok(num) = token.parse::<i64>() {
                    stack.push(num);
                } else {
                    println!("Error");
                    return;
                }
            }
        }
    }

    if stack.len() == 1 {
        println!("{}", stack.pop().unwrap());
    } else {
        println!("Error");
    }
}

// *************************************

