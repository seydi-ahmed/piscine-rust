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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct BloodType {
	pub antigen: Antigen,
	pub rh_factor: RhFactor,
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        match (&self.antigen, &other.antigen) {
            (&Antigen::A, &Antigen::A) | (&Antigen::A, &Antigen::O) |
            (&Antigen::B, &Antigen::B) | (&Antigen::B, &Antigen::O) |
            (&Antigen::AB, _) |
            (&Antigen::O, &Antigen::O)
            if self.rh_factor == RhFactor::Positive || self.rh_factor == other.rh_factor => true,
            _ => false,
        }
    }

    pub fn donors(&self) -> Vec<Self> {
        let mut donors = Vec::<Self>::new();
        for antigen in &[Antigen::O, Antigen::A, Antigen::B, Antigen::AB] {
            for rh_factor in &[RhFactor::Positive, RhFactor::Negative] {
                let donor = BloodType {
                    antigen: antigen.clone(),
                    rh_factor: rh_factor.clone(),
                };
                if self.can_receive_from(&donor) {
                    donors.push(donor);
                }
            }
        }
        donors
    }

    pub fn recipients(&self) -> Vec<BloodType> {
        let mut recipients = Vec::<Self>::new();
        for antigen in &[Antigen::O, Antigen::A, Antigen::B, Antigen::AB] {
            for rh_factor in &[RhFactor::Positive, RhFactor::Negative] {
                let recipient = BloodType {
                    antigen: antigen.clone(),
                    rh_factor: rh_factor.clone(),
                };
                if recipient.can_receive_from(self) {
                    recipients.push(recipient);
                }
            }
        }
        recipients
    }
}


// *******************************************************************
// *******************************************************************
// *******************************************************************

fn main() {
	let blood_type = BloodType {
		antigen: Antigen::O,
		rh_factor: RhFactor::Positive,
	};
	println!("recipients of O+ {:?}", blood_type.recipients());
	println!("donors of O+ {:?}", blood_type.donors());
	let another_blood_type = BloodType {
		antigen: Antigen::O,
		rh_factor: RhFactor::Positive,
	};
	println!(
		"donors of O+ can receive from {:?} {:?}",
		&another_blood_type,
		blood_type.can_receive_from(&another_blood_type)
	);
}
