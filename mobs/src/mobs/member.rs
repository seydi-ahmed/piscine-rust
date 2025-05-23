
    #[derive(Debug, Clone, PartialEq)]
    pub enum Role {
        Underboss,
        Caporegime,
        Soldier,
        Associate,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Member {
        pub name: String,
        pub role: Role,
        pub age: u8,
    }

    impl Member {
        pub fn new(name: &str, role: Role, age: u8) -> Member {
            let named = name.to_string();
            Member{
                name: named,
                role,
                age,
            }
        }

        pub fn get_promotion(&mut self) {
            match self.role {
                Role::Associate => self.role = Role::Soldier,
                Role::Soldier => self.role = Role::Caporegime,
                Role::Caporegime => self.role = Role::Underboss,
                Role::Underboss => self.role = Role::Underboss,
            }
        }
    }
