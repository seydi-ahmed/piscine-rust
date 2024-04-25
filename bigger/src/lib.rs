use std::collections::HashMap;
pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let mut max : i32 = 0;
    for (_, nombre) in h{
        if nombre > max{
            max = nombre;
        }
    }
    max
}
