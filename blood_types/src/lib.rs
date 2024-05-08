use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum RhFactor {
    Positive,
    Negative,
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

impl FromStr for Antigen {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Antigen::A),
            "AB" => Ok(Antigen::AB),
            "B" => Ok(Antigen::B),
            "O" => Ok(Antigen::O),
            _ => Err(()),
        }
    }
}

impl FromStr for RhFactor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(RhFactor::Positive),
            "-" => Ok(RhFactor::Negative),
            _ => Err(()),
        }
    }
}

impl FromStr for BloodType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let antigen_str = &s[..s.len() - 1];
        let rh_factor_str = &s[s.len() - 1..];
        let antigen = Antigen::from_str(antigen_str)?;
        let rh_factor = RhFactor::from_str(rh_factor_str)?;
        Ok(BloodType { antigen, rh_factor })
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

impl fmt::Display for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}",
            match self.antigen {
                Antigen::A => "A",
                Antigen::AB => "AB",
                Antigen::B => "B",
                Antigen::O => "O",
            },
            match self.rh_factor {
                RhFactor::Positive => "+",
                RhFactor::Negative => "-",
            }
        )
    }
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        match (&self.antigen, &other.antigen, &self.rh_factor, &other.rh_factor) {
            (&Antigen::O, _, _, _) => true,
            (_, &Antigen::AB, _, _) => true,
            (&Antigen::A, &Antigen::A, _, _) => true,
            (&Antigen::A, &Antigen::O, _, _) => true,
            (&Antigen::B, &Antigen::B, _, _) => true,
            (&Antigen::B, &Antigen::O, _, _) => true,
            _ => false,
        }
    }
    

    pub fn donors(&self) -> Vec<BloodType> {
        let mut donors = Vec::new();
        match self.antigen {
            Antigen::O => {
                donors.push(BloodType {
                    antigen: Antigen::O,
                    rh_factor: RhFactor::Positive,
                });
                donors.push(BloodType {
                    antigen: Antigen::O,
                    rh_factor: RhFactor::Negative,
                });
            }
            Antigen::AB => {
                donors.push(BloodType {
                    antigen: Antigen::AB,
                    rh_factor: RhFactor::Positive,
                });
            }
            Antigen::A => {
                donors.push(BloodType {
                    antigen: Antigen::A,
                    rh_factor: RhFactor::Positive,
                });
                donors.push(BloodType {
                    antigen: Antigen::A,
                    rh_factor: RhFactor::Negative,
                });
            }
            Antigen::B => {
                donors.push(BloodType {
                    antigen: Antigen::B,
                    rh_factor: RhFactor::Positive,
                });
                donors.push(BloodType {
                    antigen: Antigen::B,
                    rh_factor: RhFactor::Negative,
                });
            }
        }
        donors
    }

    pub fn recipients(&self) -> Vec<BloodType> {
        let mut recipients = Vec::new();
        match self.antigen {
            Antigen::AB => {
                recipients.push(BloodType {
                    antigen: Antigen::AB,
                    rh_factor: RhFactor::Positive,
                });
                recipients.push(BloodType {
                    antigen: Antigen::AB,
                    rh_factor: RhFactor::Negative,
                });
                recipients.push(BloodType {
                    antigen: Antigen::A,
                    rh_factor: RhFactor::Positive,
                });
                recipients.push(BloodType {
                    antigen: Antigen::A,
                    rh_factor: RhFactor::Negative,
                });
                recipients.push(BloodType {
                    antigen: Antigen::B,
                    rh_factor: RhFactor::Positive,
                });
                recipients.push(BloodType {
                    antigen: Antigen::B,
                    rh_factor: RhFactor::Negative,
                });
                recipients.push(BloodType {
                    antigen: Antigen::O,
                    rh_factor: RhFactor::Positive,
                });
                recipients.push(BloodType {
                    antigen: Antigen::O,
                    rh_factor: RhFactor::Negative,
                });
            }
            Antigen::A => {
                recipients.push(BloodType {
                    antigen: Antigen::A,
                    rh_factor: RhFactor::Positive,
                });
                recipients.push(BloodType {
                    antigen: Antigen::A,
                    rh_factor: RhFactor::Negative,
                });
                recipients.push(BloodType {
                    antigen: Antigen::O,
                    rh_factor: RhFactor::Positive,
                });
                recipients.push(BloodType {
                    antigen: Antigen::O,
                    rh_factor: RhFactor::Negative,
                });
            }
            Antigen::B => {
                recipients.push(BloodType {
                    antigen: Antigen::B,
                    rh_factor: RhFactor::Positive,
                });
                recipients.push(BloodType {
                    antigen: Antigen::B,
                    rh_factor: RhFactor::Negative,
                });
                recipients.push(BloodType {
                    antigen: Antigen::O,
                    rh_factor: RhFactor::Positive,
                });
                recipients.push(BloodType {
                    antigen: Antigen::O,
                    rh_factor: RhFactor::Negative,
                });
            }
            Antigen::O => {
                recipients.push(BloodType {
                    antigen: Antigen::O,
                    rh_factor: RhFactor::Positive,
                });
                recipients.push(BloodType {
                    antigen: Antigen::O,
                    rh_factor: RhFactor::Negative,
                });
            }
        }
        recipients
    }
}