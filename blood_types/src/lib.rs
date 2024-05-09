use std::cmp::{Ord, Ordering};
use std::fmt::{self, Debug, Display};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct ParseAntigenError;
#[derive(Debug, Clone)]
pub struct ParseRhFactorError;
#[derive(Debug, Clone)]
pub struct ParseBloodTypeError;
impl fmt::Display for ParseBloodTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid blood type")
    }
}
impl fmt::Display for ParseAntigenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid antigen")
    }
}
impl fmt::Display for ParseRhFactorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid rhfactor")
    }
}

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}
impl Display for Antigen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Antigen::A => write!(f, "A"),
            Antigen::AB => write!(f, "AB"),
            Antigen::B => write!(f, "B"),
            Antigen::O => write!(f, "0"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
    Positive,
    Negative,
}
impl Display for RhFactor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RhFactor::Positive => write!(f, "+"),
            RhFactor::Negative => write!(f, "-"),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

impl FromStr for Antigen {
    type Err = ParseAntigenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Antigen::A),
            "B" => Ok(Antigen::B),
            "AB" => Ok(Antigen::AB),
            "O" => Ok(Antigen::O),
            _ => Err(ParseAntigenError),
        }
    }
}

impl FromStr for RhFactor {
    type Err = ParseRhFactorError;
    // type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(RhFactor::Positive),
            "-" => Ok(RhFactor::Negative),
            _ => Err(ParseRhFactorError),
        }
    }
}

impl FromStr for BloodType {
    // type Err = ParseBloodTypeError;
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let antigen = s[..s.len() - 1]
            .parse::<Antigen>()
            .map_err(|_| ParseAntigenError)
            .unwrap();
        let rh = s[s.len() - 1..]
            .parse::<RhFactor>()
            .map_err(|_| ParseRhFactorError)
            .unwrap();

        Ok(BloodType {
            antigen,
            rh_factor: rh,
        })
    }
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        (&self.antigen, &self.rh_factor).cmp(&(&other.antigen, &other.rh_factor))
    }
}

impl Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.antigen, self.rh_factor)
    }
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        self.donors().contains(other)
    }

    pub fn donors(&self) -> Vec<Self> {
        match (&self.antigen, &self.rh_factor) {
            (Antigen::O, RhFactor::Positive) => vec![
                "O+".parse::<BloodType>().unwrap(),
                "O-".parse::<BloodType>().unwrap(),
            ],
            (Antigen::A, RhFactor::Positive) => vec![
                "A+".parse::<BloodType>().unwrap(),
                "A-".parse::<BloodType>().unwrap(),
                "O+".parse::<BloodType>().unwrap(),
                "O-".parse::<BloodType>().unwrap(),
            ],
            (Antigen::A, RhFactor::Negative) => vec![
                "A-".parse::<BloodType>().unwrap(),
                "O-".parse::<BloodType>().unwrap(),
            ],
            (Antigen::AB, RhFactor::Positive) => vec![
                "O+".parse::<BloodType>().unwrap(),
                "O-".parse::<BloodType>().unwrap(),
                "A+".parse::<BloodType>().unwrap(),
                "A-".parse::<BloodType>().unwrap(),
                "B+".parse::<BloodType>().unwrap(),
                "B-".parse::<BloodType>().unwrap(),
                "AB-".parse::<BloodType>().unwrap(),
                "AB+".parse::<BloodType>().unwrap(),
            ],
            (Antigen::AB, RhFactor::Negative) => vec![
                "AB-".parse::<BloodType>().unwrap(),
                "O-".parse::<BloodType>().unwrap(),
                "B-".parse::<BloodType>().unwrap(),
                "A-".parse::<BloodType>().unwrap(),
            ],
            (Antigen::B, RhFactor::Positive) => vec![
                "B+".parse::<BloodType>().unwrap(),
                "B-".parse::<BloodType>().unwrap(),
                "O+".parse::<BloodType>().unwrap(),
                "O-".parse::<BloodType>().unwrap(),
            ],
            (Antigen::B, RhFactor::Negative) => vec![
                "B-".parse::<BloodType>().unwrap(),
                "O-".parse::<BloodType>().unwrap(),
            ],
            (Antigen::O, RhFactor::Negative) => vec!["O-".parse::<BloodType>().unwrap()],
        }
    }

    pub fn recipients(&self) -> Vec<BloodType> {
        match (&self.antigen, &self.rh_factor) {
            (Antigen::O, RhFactor::Positive) => vec![
                "O+".parse::<BloodType>().unwrap(),
                "A+".parse::<BloodType>().unwrap(),
                "B+".parse::<BloodType>().unwrap(),
                "AB+".parse::<BloodType>().unwrap(),
            ],
            (Antigen::A, RhFactor::Positive) => vec![
                "A+".parse::<BloodType>().unwrap(),
                "AB+".parse::<BloodType>().unwrap(),
            ],
            (Antigen::A, RhFactor::Negative) => vec![
                "A-".parse::<BloodType>().unwrap(),
                "A+".parse::<BloodType>().unwrap(),
                "AB+".parse::<BloodType>().unwrap(),
                "AB-".parse::<BloodType>().unwrap(),
            ],
            (Antigen::AB, RhFactor::Positive) => vec!["AB+".parse::<BloodType>().unwrap()],
            (Antigen::AB, RhFactor::Negative) => vec![
                "AB-".parse::<BloodType>().unwrap(),
                "AB+".parse::<BloodType>().unwrap(),
            ],
            (Antigen::B, RhFactor::Positive) => vec![
                "B+".parse::<BloodType>().unwrap(),
                "AB+".parse::<BloodType>().unwrap(),
            ],
            (Antigen::B, RhFactor::Negative) => vec![
                "B-".parse::<BloodType>().unwrap(),
                "B+".parse::<BloodType>().unwrap(),
                "AB+".parse::<BloodType>().unwrap(),
                "AB-".parse::<BloodType>().unwrap(),
            ],
            (Antigen::O, RhFactor::Negative) => vec![
                "O+".parse::<BloodType>().unwrap(),
                "O-".parse::<BloodType>().unwrap(),
                "A+".parse::<BloodType>().unwrap(),
                "A-".parse::<BloodType>().unwrap(),
                "B+".parse::<BloodType>().unwrap(),
                "B-".parse::<BloodType>().unwrap(),
                "AB-".parse::<BloodType>().unwrap(),
                "AB+".parse::<BloodType>().unwrap(),
            ],
        }
    }
}
