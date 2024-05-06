pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let mut result = Vec::new();

    for num_str in s.split_whitespace() {
        let mut num = num_str.trim_end_matches('k').parse::<f64>().unwrap();
        if num_str.ends_with('k') {
            num *= 1000.0;
        }
        result.push(num as u32);
    }

    Box::new(result)
}

pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    *a
}
