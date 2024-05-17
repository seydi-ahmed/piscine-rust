//pub mod boss {
    #[derive(Debug, Clone, PartialEq)]
    pub struct Boss {
        pub name: String,
        pub age: u8,
    }

    impl Boss {
        pub fn new(name: &str, age: u8) -> Boss {
            let named = name.to_string();
            Boss{
                name: named,
                age,
            }
        }
    }
//}