use std::collections::HashMap;

pub fn slices_to_map<'a, T: std::cmp::Eq + std::hash::Hash, U>(arr1: &'a [T], arr2: &'a [U]) -> HashMap<&'a T, &'a U> {
    
    let mut map = HashMap::new();
    let size = if arr1.len() < arr2.len() {arr1.len()} else {arr2.len()};
    
    for i in 0..size{
       map.insert(&arr1[i], &arr2[i]);
    }

    map
}