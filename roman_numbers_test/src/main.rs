fn main() {
	println!("{:?}", RomanNumber::from(32));
	println!("{:?}", RomanNumber::from(9));
	println!("{:?}", RomanNumber::from(45));
	println!("{:?}", RomanNumber::from(0));
}


//********************************************************


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
	Nulla,
	I,
	V,
	X,
	L,
	C,
	D,
	M,
}

#[derive(Debug, Clone, Debug, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
    fn from (num: u32) -> Self {
        match num {
            1 => RomanDigit::I,
            _ => RomanDigit::Nulla,
        }
    }
}

impl From<u32> for RomanNumber {
    fn from (num: u32) -> Self {
        if n == 0 {
            return RomanNumber(vec![RomanDigit::Nulla]);
        }

        use RomanDigit::*;
        let conversions = [
            (1000, vec![M]),
            (900, vec![C, M]),
            (500, vec![D]),
            (400, vec![C, D]),
            (100, vec![C]),
            (90, vec![X, C]),
            (50, vec![L]),
            (40, vec![X, L]),
            (10, vec![X]),
            (9, vec![I, X]),
            (5, vec![V]),
            (4, vec![I, V]),
            (1, vec![I]),
        ];
    }
}