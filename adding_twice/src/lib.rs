pub fn add_curry(arg: i32) -> impl Fn(i32) -> i32 {
    move |x| arg + x
}

pub fn twice<T,F>(f: F) -> impl Fn(T) -> T
where
    F: Fn(T) -> T,
    T: Copy
 {
    move |x| f(f(x))
 }
