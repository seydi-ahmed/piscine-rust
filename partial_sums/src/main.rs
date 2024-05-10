pub fn parts_sums(arr: &[u64]) -> Vec<u64> {
    let mut sums = vec![0; arr.len() + 1];
    
    for i in 0..arr.len() {
        sums[i + 1] = sums[i] + arr[i];
    }
    
    sums.reverse();

    sums
}

fn main() {
    println!(
        "Partial sums of [5, 18, 3, 23] is : {:?}",
        parts_sums(&[5, 18, 3, 23])
    );
}