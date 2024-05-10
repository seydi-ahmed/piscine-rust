fn main() {
	println!("{:?}", RomanNumber::from(32));
	println!("{:?}", RomanNumber::from(9));
	println!("{:?}", RomanNumber::from(45));
	println!("{:?}", RomanNumber::from(0));
}

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
    fn from (num : u32) -> Self {
        match num {
            0 => RomanDigit::Nulla,
            1 => RomanDigit::I,
            5 => RomanDigit::V,
            10 => RomanDigit::X,
            50 => RomanDigit::L,
            100 => RomanDigit::C,
            500 => RomanDigit::D,
            1000 => RomanDigit::M,
            _ => panic!(),
        }
    }
}

impl From<u32> for RomanNumber {
    fn from (mut num : u32) -> Self {
        if num == 0 {
            return RomanNumber(vec![RomanDigit::Nulla]);
        }

        let mut result = Vec::new();
        let div = vec![1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];

        for (i, n) in div.iter().enumerate() {
            while n <= &num {
                if i % 2 == 0 {
                    result.push(RomanDigit::from(*n));
                } else {
                    let rem = div[i - 1] - div[i];
                    result.push(RomanDigit::from(rem));
                    result.push(RomanDigit::from(div[i - 1]));
                }
                num -= n;}
        }

        RomanNumber(result)
    }
}