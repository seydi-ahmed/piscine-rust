pub fn sum(a: u8, b: u8) -> u8 {
    let c : u8 = a + b;
    c
}

pub fn diff(a: i16, b: i16) -> i16 {
    let c : i16 = a - b;
    c
}

pub fn pro(a: i8, b: i8) -> i8 {
    let c : i8 = a * b;
    c
}

pub fn quo(a: f32, b: f32) -> f32 {
    a / b
}

pub fn rem(a: f32, b: f32) -> f32 {
    a % b
}


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
