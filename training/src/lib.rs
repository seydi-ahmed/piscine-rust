// *************************************

pub fn reverse_it(v: i32) -> String {
    let mut v1: i32 = v.clone();
    let mut new_str: String = String::new();

    if v * (-1) >= 0 {
        v1 = -v1;
        new_str.push_str(&"-".to_string());
    }

    let mut tab: Vec<i32> = Vec::new();

    while v1 > 0 {
        let q: i32 = v1 / 10;
        let r: i32 = v1 % 10;
        tab.push(r);
        v1 = q;
    }

    for nombre in tab.clone() {
        new_str.push_str(&nombre.to_string());
    }

    for nombre in tab.iter().rev() {
        new_str.push_str(&nombre.to_string());
    }

    new_str
}

// *************************************

use std::collections::HashMap;
pub fn counting_words(words: &str) -> HashMap<String, u32> {
    let mut word_count: HashMap<String, u32> = HashMap::new();

    let words = words.replace(|c: char| !c.is_alphanumeric() && c != '\'', " ");

    for word in words.split_whitespace() {
        let word = word.to_lowercase();

        // if (word.start_with('\'') && word.end_with('\'')) {
        //     word.remove(0);
        //     word.remove(word.len() -1);
        // }

        if word != "" {
            *word_count.entry(word).or_insert(0) += 1;
        }
    }

    word_count
}

// *************************************

pub fn parts_sums(arr: &[u64]) -> Vec<u64> {
    let mut sums = vec![0; arr.len() + 1];
    
    for i in 0..arr.len() {
        sums[i + 1] = sums[i] + arr[i];
    }
    
    sums.reverse();

    sums
}

// *************************************

pub fn next_prime(nbr: u64) -> u64 {
    let mut num = nbr;

    while !is_prime(num) {
        num += 1;
    }
    
    num
}

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    let mut i = 3;

    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}


pub fn prev_prime(nbr: u64) -> u64 {
    if nbr <= 2 {
        return 0;
    }
    
    let mut num = nbr;
    
    while !is_prime(num) {
        num -= 1;
    }
    
    num
}

// *************************************

pub fn inv_pyramid(v: String, i: u32) -> Vec<String> {
    (1..=i as usize)
        .chain((1..i as usize).rev())
        .map(|j| format!("{}{}", " ".repeat(j), v.repeat(j)))
        .collect()
}

// *************************************

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

