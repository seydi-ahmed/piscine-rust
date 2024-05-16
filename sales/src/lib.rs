#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}

impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }

    pub fn get_price(&self, product_name: &str) -> Option<f32> {
        self.products.iter().find_map(|(name, price)| {
            if name == product_name {
                Some(*price)
            } else {
                None
            }
        })
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

    pub fn insert_item(&mut self, store: &Store, item_name: String) {
        if let Some(price) = store.get_price(&item_name) {
            self.items.push((item_name, price));
        }
    }

    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut prices: Vec<f32> = self.items.iter().map(|(_, price)| *price).collect();
        prices.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let total_discount: f32 = prices
            .chunks_exact(3)
            .map(|chunk| chunk[0])
            .sum();

        let total_price: f32 = prices.iter().sum();
        let adjustment = total_discount / total_price;

        self.receipt = prices
            .iter()
            .map(|&price| (price - price * adjustment).round_to(2))
            .collect();
        self.receipt.clone()
    }
}

trait RoundTo {
    fn round_to(&self, digits: u32) -> f32;
}

impl RoundTo for f32 {
    fn round_to(&self, digits: u32) -> f32 {
        let factor = 10_f32.powi(digits as i32);
        (self * factor).round() / factor
    }
}
