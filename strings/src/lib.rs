pub fn char_length(s: &str) -> usize {
    let mut res : usize = 0;
    for _ in s.chars(){
        res += 1;
    }

    res
}
