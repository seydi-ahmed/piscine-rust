use std::collections::BTreeSet;

pub fn flatten_tree<T: Clone + Ord>(tree: &BTreeSet<T>) -> Vec<T> {
    tree.iter().cloned().collect()
}

// Pour tester le code
fn main() {
    let mut tree = BTreeSet::new();
    tree.insert(34);
    tree.insert(0);
    tree.insert(9);
    tree.insert(30);
    println!("{:?}", flatten_tree(&tree));

    let mut tree = BTreeSet::new();
    tree.insert("Slow");
    tree.insert("kill");
    tree.insert("will");
    tree.insert("Horses");
    println!("{:?}", flatten_tree(&tree));
}