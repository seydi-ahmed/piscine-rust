pub use crate::mobs::member::Member as crateMember;
pub use crate::mobs::member::Role as crateRole;
pub mod boss;
pub mod member;


#[derive(Debug, Clone, PartialEq)]
pub struct Mob {
    pub name: String,
    pub boss: boss::Boss,
    pub members: Vec<member::Member>,
    pub cities: Vec<(String, u8)>,
    pub wealth: u32,
}

impl Mob {
    pub fn recruit(&mut self, name: &str, age: u8) {
        self.members.push(crateMember::new(name, crateRole::Associate,age))
    }

    pub fn attack(&mut self, mob:&mut Mob) {
        if mob.members.len() == 0 {
            self.take_wealth(mob);
        } else if self.members.len() == 0 {
            mob.take_wealth(self)
        }

        let (mut attacker, mut defender) = (0, 0);
        for member in &self.members {
            match member.role {
                crateRole::Associate => attacker += 1,
                crateRole::Soldier => attacker += 2,
                crateRole::Caporegime => attacker += 3,
                crateRole::Underboss => attacker += 4,
            }
        }
        for member in &mob.members {
            match member.role {
                crateRole::Associate => defender += 1,
                crateRole::Soldier => defender += 2,
                crateRole::Caporegime => defender += 3,
                crateRole::Underboss => defender += 4,
            }
        }

        if defender >= attacker {
            self.members.pop();
            if self.members.len() == 0 {
                mob.take_wealth(self)
            }
        } else {
            mob.members.pop();
            if mob.members.len() == 0 {
                self.take_wealth(mob)
            }
        }
    }

    pub fn take_wealth(&mut self, mob: &mut Mob) {
        self.cities.append(&mut mob.cities);
        self.wealth += mob.wealth;
        mob.cities = vec![];
        mob.wealth = 0 as u32;
    }

    pub fn steal(&mut self, mob: &mut Mob, value: u32) {
        if value <= mob.wealth {
            self.wealth += value;
            mob.wealth -= value;
        } else {
            self.wealth += mob.wealth;
            mob.wealth = 0;
        }
    }

    pub fn conquer_city(&mut self, mobs: Vec<Mob>, name: String, value: u8) {
        let mut non_conquered = true;
        for mob in mobs {
            for city in mob.cities {
                if city.0 == name {
                    non_conquered = false;
                    break;
                } 
            }
        }
        if non_conquered {
            self.cities.push((name, value));
        }
    }
    
}

