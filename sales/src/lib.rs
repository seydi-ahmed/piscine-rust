use core::f32;
use std::vec;

#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}

impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>,
}

impl Cart {
    pub fn new() -> Cart {
        Cart {
            items: vec![],
            receipt: vec![],
        }
    }

    pub fn insert_item(&mut self, s: &Store, ele: String) {
        match s.products.iter().find(|x| x.0 == ele) {
            Some(e) => self.items.push((ele, e.1)),
            None => todo!(),
        }
    }

    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let l = self.items.len();
        let items: Vec<f32> = self.items.iter().map(|e| e.1).collect();
        let mut cp = items.clone();

        if l < 3 {
            return items;
        } else {
            // somme total
            let sum1: f32 = cp.iter().sum();
            cp.sort_by(|a, b| a.partial_cmp(b).unwrap());

            // la somme des articles offerts
            let sum2: f32 = cp[0..l / 3].iter().sum();

            let percentage = (sum2 / sum1) * 100.0;
            let apply_percentage = |x| {
                let f: f32 = x - x * percentage / 100.0;
                (f * 100.0).round() / 100.0
            };
            let res: Vec<f32> = cp.iter().map(apply_percentage).collect();
            self.receipt = res.clone();
            res
        }
    }
}
