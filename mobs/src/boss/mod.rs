// mobs/src/boss/mod.rs
#[derive(Debug, Clone, PartialEq)]
pub struct Boss {
    pub name: String,
    pub age: u8,
}

impl Boss {
    pub fn new(name: String, age: u8) -> Self {
        Boss { name, age }
    }
}
