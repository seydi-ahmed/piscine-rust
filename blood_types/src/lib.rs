// lib.rs

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

use std::cmp::{Ord, Ordering};

use std::str::FromStr;

impl FromStr for Antigen {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Antigen::A),
            "AB" => Ok(Antigen::AB),
            "B" => Ok(Antigen::B),
            "O" => Ok(Antigen::O),
            _ => Err("Invalid antigen".to_string()),
        }
    }
}

impl FromStr for RhFactor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(RhFactor::Positive),
            "-" => Ok(RhFactor::Negative),
            _ => Err("Invalid Rh factor".to_string()),
        }
    }
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.antigen != other.antigen {
            self.antigen.cmp(&other.antigen)
        } else {
            self.rh_factor.cmp(&other.rh_factor)
        }
    }
}

impl FromStr for BloodType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split("").collect();
        if parts.len() != 3 || (parts[2] != "+" && parts[2] != "-") {
            return Err("Invalid blood type format. Expected format: Antigen+/-".to_string());
        }

        let antigen = Antigen::from_str(parts[0])?;
        let rh_factor = RhFactor::from_str(parts[1])?;

        Ok(BloodType { antigen, rh_factor })
    }
}


use std::fmt::{self, Debug};

impl Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rh_str = match self.rh_factor {
            RhFactor::Positive => "+",
            RhFactor::Negative => "-",
        };
        write!(f, "{:?}{}", self.antigen, rh_str)
    }
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        match (self.antigen.clone(), other.antigen.clone(), self.rh_factor.clone(), other.rh_factor.clone()) {
            (Antigen::AB, _, _, _) => true,
            (Antigen::A, Antigen::A, _, _) => true,
            (Antigen::A, Antigen::O, _, _) => true,
            (Antigen::B, Antigen::B, _, _) => true,
            (Antigen::B, Antigen::O, _, _) => true,
            (Antigen::O, _, _, _) => true,
            _ => false,
        }
    }

    pub fn donors(&self) -> Vec<Self> {
        let mut donors = Vec::new();
        let antigen = self.antigen.clone();
        let rh_factor = self.rh_factor.clone();
        match (antigen, rh_factor) {
            (Antigen::A, RhFactor::Positive) => {
                donors.push(BloodType { antigen: Antigen::A, rh_factor: RhFactor::Positive });
                donors.push(BloodType { antigen: Antigen::A, rh_factor: RhFactor::Negative });
            },
            (Antigen::A, RhFactor::Negative) => {
                donors.push(BloodType { antigen: Antigen::A, rh_factor: RhFactor::Negative });
            },
            (Antigen::B, RhFactor::Positive) => {
                donors.push(BloodType { antigen: Antigen::B, rh_factor: RhFactor::Positive });
                donors.push(BloodType { antigen: Antigen::B, rh_factor: RhFactor::Negative });
            },
            (Antigen::B, RhFactor::Negative) => {
                donors.push(BloodType { antigen: Antigen::B, rh_factor: RhFactor::Negative });
            },
            (Antigen::AB, RhFactor::Positive) => {
                donors.push(BloodType { antigen: Antigen::A, rh_factor: RhFactor::Positive });
                donors.push(BloodType { antigen: Antigen::A, rh_factor: RhFactor::Negative });
                donors.push(BloodType { antigen: Antigen::B, rh_factor: RhFactor::Positive });
                donors.push(BloodType { antigen: Antigen::B, rh_factor: RhFactor::Negative });
                donors.push(BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Positive });
                donors.push(BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Negative });
            },
            (Antigen::AB, RhFactor::Negative) => {
                donors.push(BloodType { antigen: Antigen::A, rh_factor: RhFactor::Negative });
                donors.push(BloodType { antigen: Antigen::B, rh_factor: RhFactor::Negative });
                donors.push(BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Negative });
            },
            (Antigen::O, RhFactor::Positive) => {
                donors.push(BloodType { antigen: Antigen::O, rh_factor: RhFactor::Positive });
                donors.push(BloodType { antigen: Antigen::O, rh_factor: RhFactor::Negative });
            },
            (Antigen::O, RhFactor::Negative) => {
                donors.push(BloodType { antigen: Antigen::O, rh_factor: RhFactor::Negative });
            },
        }
        donors
    }

    pub fn recipients(&self) -> Vec<BloodType> {
        let mut recipients = Vec::new();
        let antigen = self.antigen.clone();
        let rh_factor = self.rh_factor.clone();
        match (antigen, rh_factor) {
            (Antigen::A, RhFactor::Positive) => {
                recipients.push(BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Positive });
                recipients.push(BloodType { antigen: Antigen::A, rh_factor: RhFactor::Positive });
            },
            (Antigen::A, RhFactor::Negative) => {
                recipients.push(BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Positive });
                recipients.push(BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Negative });
                recipients.push(BloodType { antigen: Antigen::A, rh_factor: RhFactor::Positive });
                recipients.push(BloodType { antigen: Antigen::A, rh_factor: RhFactor::Negative });
            },
            (Antigen::B, RhFactor::Positive) => {
                recipients.push(BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Positive });
                recipients.push(BloodType { antigen: Antigen::B, rh_factor: RhFactor::Positive });
            },
            (Antigen::B, RhFactor::Negative) => {
                recipients.push(BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Positive });
                recipients.push(BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Negative });
                recipients.push(BloodType { antigen: Antigen::B, rh_factor: RhFactor::Positive });
                recipients.push(BloodType { antigen: Antigen::B, rh_factor: RhFactor::Negative });
            },
            (Antigen::AB, RhFactor::Positive) => {
                recipients.push(BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Positive });
            },
            (Antigen::AB, RhFactor::Negative) => {
                recipients.push(BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Positive });
                recipients.push(BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Negative });
            },
            (Antigen::O, RhFactor::Positive) => {
                recipients.push(BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Positive });
                recipients.push(BloodType { antigen: Antigen::A, rh_factor: RhFactor::Positive });
                recipients.push(BloodType { antigen: Antigen::B, rh_factor: RhFactor::Positive });
                recipients.push(BloodType { antigen: Antigen::O, rh_factor: RhFactor::Positive });
            },
            (Antigen::O, RhFactor::Negative) => {
                recipients.push(BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Positive });
                recipients.push(BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Negative });
                recipients.push(BloodType { antigen: Antigen::A, rh_factor: RhFactor::Positive });
                recipients.push(BloodType { antigen: Antigen::A, rh_factor: RhFactor::Negative });
                recipients.push(BloodType { antigen: Antigen::B, rh_factor: RhFactor::Positive });
                recipients.push(BloodType { antigen: Antigen::B, rh_factor: RhFactor::Negative });
                recipients.push(BloodType { antigen: Antigen::O, rh_factor: RhFactor::Positive });
                recipients.push(BloodType { antigen: Antigen::O, rh_factor: RhFactor::Negative });
            },
        }
        recipients
    }
}
