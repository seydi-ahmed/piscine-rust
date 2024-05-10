pub fn parts_sums(arr: &[u64]) -> Vec<u64> {
    let mut result : Vec<u64> = vec![0; arr.len() + 1];

    for i in 0..arr.len() {
        result[i+1] = result[i] + arr[i];
    }

    result.reverse();

    result
}

fn main() {
    println!(
        "Partial sums of [5, 18, 3, 23] is : {:?}",
        parts_sums(&[5, 18, 3, 23])
    );
}