pub fn fibonacci(n: u32) -> u32 {
    if n == 1{
        return 1;
    }
    if n == 2{
        return 1;
    }
    if n == 3{
        return 2;
    }
    if n == 4{
        return 3;
    }
    return fibonacci(n-1) + fibonacci(n-2);
}

fn main() {
    println!("The element in the position {} in fibonacci series is {}",2, fibonacci(2));
    println!("The element in the position {} in fibonacci series is {}",4, fibonacci(4));
    println!("The element in the position {} in fibonacci series is {}",22, fibonacci(22));
    println!("The element in the position {} in fibonacci series is {}", 20, fibonacci(20));
}