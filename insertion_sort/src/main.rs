pub fn insertion_sort(slice: &mut [i32], steps: usize) {
    (slice.split_at_mut(steps+1).0).sort()
}

fn main() {
    let mut target = [5, 3, 7, 2, 1, 6, 8, 4];
    // executes the first iteration of the algorithm
    insertion_sort(&mut target, 1);
    println!("{:?}", target);

    let mut target = [5, 3, 7, 2, 1, 6, 8, 4];
    let len = target.len();
    // executes len - 1 iterations of the algorithm
    // i.e. sorts the slice
    insertion_sort(&mut target, len - 1);
    println!("{:?}", target);
}