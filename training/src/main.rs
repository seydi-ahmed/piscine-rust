use std::collections::HashMap;

pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let mut max : i32 = 0;

    for (_, elem) in h {
        if elem > max {
            max = elem;
        }
    }


    max
}


fn main() {

    let mut hash = HashMap::new();
    hash.insert("Daniel", 122);
    hash.insert("Ashley", 333);
    hash.insert("Katie", 334);
    hash.insert("Robert", 14);

    println!("The biggest of the elements in the HashMap is {}", bigger(hash));
}