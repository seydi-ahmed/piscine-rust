// // use crate::mobs::boss::Boss;
// // use crate::mobs::member::{Member, Role};

// use crate::boss::Boss;
// use crate::member::{Member, Role};

// #[derive(Debug, Clone, PartialEq)]
// pub struct Mob {
//     pub name: String,
//     pub boss: Boss,
//     pub members: Vec<Member>,
//     pub cities: Vec<(String, u8)>,
//     pub wealth: u32,
// }

// impl Mob {
//     pub fn recruit(name: &str, age: u8) -> Member {
//         Member::new(name, Role::Associate, age)
//     }

//     pub fn attack(attacker: &mut Mob, defender: &mut Mob) {
//         let attacker_power = attacker.members.iter().map(|m| mob_score(&m.role)).sum::<u8>();
//         let defender_power = defender.members.iter().map(|m| mob_score(&m.role)).sum::<u8>();

//         if attacker_power > defender_power {
//             defender.members.pop();
//             if defender.members.is_empty() {
//                 attacker.cities.append(&mut defender.cities);
//                 attacker.wealth += defender.wealth;
//                 defender.wealth = 0;
//             }
//         } else if attacker_power < defender_power {
//             attacker.members.pop();
//             if attacker.members.is_empty() {
//                 defender.cities.append(&mut attacker.cities);
//                 defender.wealth += attacker.wealth;
//                 attacker.wealth = 0;
//             }
//         }
//     }

//     pub fn steal(mob: &mut Mob, target: &mut Mob, value: u32) {
//         let stolen = value.min(target.wealth);
//         target.wealth -= stolen;
//         mob.wealth += stolen;
//     }

//     pub fn conquer_city(mobs: &mut Vec<Mob>, city_name: &str, city_value: u8) {
//         for mob in mobs {
//             if !mob.cities.iter().any(|&(ref name, _)| name == city_name) {
//                 mob.cities.push((String::from(city_name), city_value));
//                 break;
//             }
//         }
//     }
// }

// fn mob_score(role: &Role) -> u8 {
//     match role {
//         Role::Underboss => 4,
//         Role::Caporegime => 3,
//         Role::Soldier => 2,
//         Role::Associate => 1,
//     }
// }


