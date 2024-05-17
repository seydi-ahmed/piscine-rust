use std::env;

fn is_correctly_bracketed(expression: &str) -> bool {
    let mut stack = Vec::new();

    for ch in expression.chars() {
        match ch {
            '(' | '[' | '{' => stack.push(ch),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            _ => {} // Ignore other characters
        }
    }

    stack.is_empty()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        for arg in &args[1..] {
            if is_correctly_bracketed(arg) {
                println!("OK");
            } else {
                println!("Error");
            }
        }
    }
}
