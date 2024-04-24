pub fn delete_and_backspace(s: &mut String) {
    let mut result = String::new();
    let mut ignore_next = false;

    for c in s.chars() {
        match c {
            '-' => {
                result.pop(); // Remove the last character if it's a backspace
            }
            '+' => {
                ignore_next = true; // Set flag to ignore the next character if it's a delete
            }
            _ => {
                if !ignore_next {
                    result.push(c); // Add the character to the result unless it's flagged to be ignored
                } else {
                    ignore_next = false; // Reset the flag if the character is not ignored
                }
            }
        }
    }

    *s = result; // Update the original string with the processed result
}


pub fn do_operations(v: &mut Vec<String>) {
    for equation in v.iter_mut() {
        let result: i32 = eval(equation); // Evaluate the equation
        *equation = result.to_string(); // Update the equation with the result
    }
}

// Helper function to evaluate simple addition and subtraction equations
fn eval(equation: &String) -> i32 {
    let mut result = 0;
    let mut operator = '+';
    let mut num = String::new();

    for c in equation.chars() {
        if c.is_numeric() {
            num.push(c); // Add numeric characters to num
        } else {
            if !num.is_empty() {
                // Perform addition or subtraction based on the previous operator
                let n: i32 = num.parse().unwrap();
                match operator {
                    '+' => result += n,
                    '-' => result -= n,
                    _ => (),
                }
                num.clear(); // Clear num for the next number
            }
            operator = c; // Update operator
        }
    }

    // Perform the last addition or subtraction with the remaining number
    let n: i32 = num.parse().unwrap();
    match operator {
        '+' => result += n,
        '-' => result -= n,
        _ => (),
    }

    result
}
