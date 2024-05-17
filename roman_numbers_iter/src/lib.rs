use crate::RomanDigit::*;

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
	fn from(value: u32) -> Self {
		match value {
            0 => Nulla,
            1 => I,
            5 => V,
            10 => X,
            50 => L,
            100 => C,
            500 => D,
            1000 => M,
            _ => todo!(),
        }
	}
}

impl From<u32> for RomanNumber {
	fn from(value: u32) -> Self {
        print!("value {:?}", value);
		let mut num = value;
		let mut roman_digits = Vec::new();
        let mut index = 0;
        let  powers_of_ten = vec![1000, 100, 10, 1];

		if num == 0 {
            print!("value de {:?}", value);
            return RomanNumber(vec![RomanDigit::Nulla]);
        }

        while num > 0 {
            let digit = num / powers_of_ten[index];
            if digit > 0 {
                match digit {
                    1..=3 => {
                        for _ in 0..digit {
                            roman_digits.push(RomanDigit::from(powers_of_ten[index]));
                        }
                    }
                    4 => {
                        roman_digits.push(RomanDigit::from(powers_of_ten[index]));
                        roman_digits.push(RomanDigit::from(powers_of_ten[index] * 5));
                    }
                    5 => {
                        roman_digits.push(RomanDigit::from(powers_of_ten[index] * 5));
                    }
                    6..=8 => {
                        roman_digits.push(RomanDigit::from(powers_of_ten[index] * 5));
                        for _ in 0..(digit - 5) {
                            roman_digits.push(RomanDigit::from(powers_of_ten[index]));
                        }
                    }
                    9 => {
                        roman_digits.push(RomanDigit::from(powers_of_ten[index]));
                        roman_digits.push(RomanDigit::from(powers_of_ten[index - 1]));
                    }
                    _ => unreachable!(),
                }
            }
            num %= powers_of_ten[index];
            index += 1;
        }
        
        RomanNumber(roman_digits)
	}
	
}
impl RomanNumber {
    // Fonction pour convertir un vecteur de chiffres romains en u32
    fn to_u32(&self) -> u32 {
        let mut result = 0;
        let mut prev_value = 0;

        for digit in &self.0 {
            let value = match digit {
                Nulla => 0,
                I => 1,
                V => 5,
                X => 10,
                L => 50,
                C => 100,
                D => 500,
                M => 1000,
            };
            result += if value > prev_value {
                // Si la valeur actuelle est plus grande que la précédente,
                // cela signifie que nous avons un chiffre romain composé.
                // Dans ce cas, nous devons soustraire deux fois la valeur précédente.
                value - 2 * prev_value
            } else {
                value
            };
            prev_value = value;
        }

        result
    }
}

impl Iterator for RomanNumber {
    type Item = RomanNumber;

     fn next(&mut self) -> Option<Self::Item> {
        // Convertir la liste de chiffres romains en nombre entier
        let mut value = self.to_u32();

        // Incrémenter le nombre
        value += 1;

        // Convertir le nouveau nombre en une RomanNumber
        *self = RomanNumber::from(value);

        // Renvoyer la RomanNumber mise à jour
        Some(self.clone())
    }
}