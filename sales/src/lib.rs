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
    items: Vec<(String, f32)>,
    receipt: Vec<f32>,
}

impl Cart {
    pub fn new() -> Cart {
        Cart {
            items: Vec::new(),
            receipt: Vec::new(),
        }
    }

    pub fn insert_item(&mut self, store: &Store, ele: String) {
        if let Some(item) = store.products.iter().find(|(name, _)| name == &ele) {
            self.items.push(item.clone());
        }
    }

    pub fn generate_receipt(&mut self) -> Vec<f32> {
        // Sort items by price in ascending order
        self.items.sort_by(|(_, price1), (_, price2)| price1.partial_cmp(price2).unwrap());

        // Apply the "buy three, get one free" promotion
        let mut adjusted_prices = Vec::new();
        let mut count = 0;

        for (name, price) in &self.items {
            adjusted_prices.push(*price);
            count += 1;

            // Check if three items have been added
            if count == 3 {
                // Adjust the price of the cheapest item to zero
                adjusted_prices[adjusted_prices.len() - 3] = 0.0;
                count = 0; // Reset the count for the next three items
            }
        }

        // Update receipt field
        self.receipt = adjusted_prices.clone();

        // Return adjusted prices
        adjusted_prices
    }
}
