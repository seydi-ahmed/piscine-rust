use hashing::*;

fn main() {
    println!("Hello, world!");
    let v = vec![4, 7, 5, 2, 5, 1, 3];
    println!("mean {}", mean(&v));
    println!("median {}", hashing::median(&v));
    println!("mode {}", hashing::mode(&v));
}