pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    println!("arr: {:?}", arr);
    if arr.len() <= 1 {
        return vec![];
    }
    let mut products = Vec::with_capacity(arr.len());

    for i in 0..arr.len() {
        let mut product = 1;
        for j in 0..arr.len() {
            if j != i {
                product *= arr[j];
            }
        }
        products.push(product);
    }
    products
}
