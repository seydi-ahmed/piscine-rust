// Dans le fichier lib.rs

pub mod mobs {
    pub mod boss {
        #[derive(Debug, Clone, PartialEq)]
        pub struct Boss {
            pub name: String,
            pub age: u8,
        }

        impl Boss {
            pub fn new(name: &str, age: u8) -> Boss {
                Boss {
                    name: name.to_string(),
                    age,
                }
            }
        }
    }

    pub mod member {
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
                Member {
                    name: name.to_string(),
                    role,
                    age,
                }
            }

            pub fn get_promotion(&mut self) {
                match self.role {
                    Role::Associate => self.role = Role::Soldier,
                    Role::Soldier => self.role = Role::Caporegime,
                    Role::Caporegime => self.role = Role::Underboss,
                    _ => {}
                }
            }
        }
    }

    use boss::Boss;
    use member::{Member, Role};

    #[derive(Debug, Clone, PartialEq)]
    pub struct Mob {
        pub name: String,
        pub boss: Boss,
        pub members: Vec<Member>,
        pub cities: Vec<(String, u8)>,
        pub wealth: u32,
    }

    impl Mob {
        pub fn recruit(name: &str, age: u8) -> Member {
            Member::new(name, Role::Associate, age)
        }

        pub fn attack(&mut self, other_mob: &mut Mob) {
            let self_score = self.members.iter().map(|member| match member.role {
                Role::Underboss => 4,
                Role::Caporegime => 3,
                Role::Soldier => 2,
                Role::Associate => 1,
            }).sum::<u32>();

            let other_score = other_mob.members.iter().map(|member| match member.role {
                Role::Underboss => 4,
                Role::Caporegime => 3,
                Role::Soldier => 2,
                Role::Associate => 1,
            }).sum::<u32>();

            if self_score > other_score {
                if let Some(member) = other_mob.members.pop() {
                    self.members.push(member);
                }
            } else if self_score < other_score {
                if let Some(member) = self.members.pop() {
                    other_mob.members.push(member);
                }
            } else {
                // Draw
                self.members.pop();
            }

            if self.members.is_empty() {
                // Transfer cities and wealth
                self.cities.append(&mut other_mob.cities);
                self.wealth += other_mob.wealth;
            } else if other_mob.members.is_empty() {
                // Transfer cities and wealth
                other_mob.cities.append(&mut self.cities);
                other_mob.wealth += self.wealth;
            }
        }

        pub fn steal(&mut self, other_mob: &mut Mob, value: u32) {
            let stolen = value.min(other_mob.wealth);
            other_mob.wealth -= stolen;
            self.wealth += stolen;
        }

        pub fn conquer_city(mobs: &mut Vec<Mob>, city_name: String, value: u8) {
            let mut city_taken = false;
            for mob in mobs.iter_mut() {
                if mob.cities.iter().any(|(name, _)| name == &city_name) {
                    city_taken = true;
                    break;
                }
            }

            if !city_taken {
                for mob in mobs.iter_mut() {
                    mob.cities.push((city_name.clone(), value));
                }
            }
        }
    }
}

