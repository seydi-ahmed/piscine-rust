pub fn divide(x: i32, y: i32) -> (i32, i32) {
    let q : i32 = x / y;
    let r : i32 = x % y;
    (q,r)
}