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
            items: Vec::new(),
            receipt: Vec::new(),
        }
    }

    pub fn insert_item(&mut self, store: &Store, ele: String) {
        if let Some((_, price)) = store.products.iter().find(|&&(ref name, _)| name == &ele) {
            self.items.push((ele, *price));
        }
    }

    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut sorted_prices: Vec<f32> = self.items.iter().map(|&(_, price)| price).collect();
        sorted_prices.sort_by(|a, b| b.partial_cmp(a).unwrap());

        let mut discounted_prices: Vec<f32> = Vec::new();
        let mut counter = 0;
        let mut total_discounted = 0.0;

        for price in sorted_prices {
            counter += 1;
            discounted_prices.push(price);
            total_discounted += price;
            if counter == 3 {
                discounted_prices.pop();
                total_discounted -= price;
                counter = 0;
            }
        }

        let reduction_factor = total_discounted / self.items.iter().map(|&(_, price)| price).sum::<f32>();
        self.receipt = self
            .items
            .iter()
            .map(|&(_, price)| (price * reduction_factor * 100.0).round() / 100.0)
            .collect();

        self.receipt.clone()
    }

    // pub fn get_receipt(&self) -> &[f32] {
    //     &self.receipt
    // }
}
