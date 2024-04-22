pub fn sum(a: u8, b: u8) -> String {
    if let Some(c) = a.checked_add(b) {
        c.to_string()
    } else {
        String::from("ERROR: attempt to add with overflow")
    }
}

pub fn diff(a: i16, b: i16) -> String {
    if let Some(c) = a.checked_sub(b) {
        c.to_string()
    } else {
        String::from("ERROR: attempt to add with overflow")
    }
}

pub fn pro(a: i8, b: i8) -> String {
    if let Some(c) = a.checked_mul(b) {
        c.to_string()
    } else {
        String::from("ERROR: attempt to add with overflow")
    }
}

pub fn quo(a: f32, b: f32) -> f32 {
    a / b
}

pub fn rem(a: f32, b: f32) -> f32 {
    a % b
}


// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
