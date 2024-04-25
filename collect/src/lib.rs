pub fn bubble_sort(vec: &mut Vec<i32>) {
    let mut swapped = true;

    while swapped {
        swapped = false;
        for i in 0..vec.len()-1 {
            if vec[i] > vec[i + 1] {
                vec.swap(i, i + 1);
                swapped = true;
            }
        }
    }
}