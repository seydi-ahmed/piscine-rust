
pub fn add_curry(arg: i32)-> impl Fn(i32) -> i32{
    move |x| arg + x
} 