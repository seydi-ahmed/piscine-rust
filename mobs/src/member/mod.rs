// mobs/src/member/mod.rs
#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    Underboss,
    Caporegime,
    Soldier,
    Associate,
}

// mobs/src/member/mod.rs
#[derive(Debug, Clone, PartialEq)]
pub struct Member {
    pub name: String,
    pub role: Role,
    pub age: u8,
}

impl Member {
    pub fn new(name: String, role: Role, age: u8) -> Self {
        Member { name, role, age }
    }

    pub fn get_promotion(&mut self) {
        match self.role {
            Role::Associate => self.role = Role::Soldier,
            Role::Soldier => self.role = Role::Caporegime,
            Role::Caporegime => self.role = Role::Underboss,
        }
    }
}
