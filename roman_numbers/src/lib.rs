use std::fmt;

use crate::RomanDigit::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    IV,
    V,
    IX,
    X,
    XL,
    L,
    XC,
    C,
    CD,
    D,
    CM,
    M,
}

impl fmt::Display for RomanDigit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match *self {
            RomanDigit::Nulla => "",
            RomanDigit::I => "I",
            RomanDigit::IV => "IV",
            RomanDigit::V => "V",
            RomanDigit::IX => "IX",
            RomanDigit::X => "X",
            RomanDigit::XL => "XL",
            RomanDigit::L => "L",
            RomanDigit::XC => "XC",
            RomanDigit::C => "C",
            RomanDigit::CD => "CD",
            RomanDigit::D => "D",
            RomanDigit::CM => "CM",
            RomanDigit::M => "M",
        };
        write!(f, "{}", symbol)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl fmt::Display for RomanNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let roman_str: Vec<String> = self.0.iter().map(|digit| format!("{}", digit)).collect();
        write!(f, "RomanNumber([{}])", roman_str.join(", "))
    }
}

impl From<u32> for RomanDigit {
    fn from(num: u32) -> Self {
        match num {
            0 => RomanDigit::Nulla,
            1 => RomanDigit::I,
            4 => RomanDigit::IV,
            5 => RomanDigit::V,
            9 => RomanDigit::IX,
            10 => RomanDigit::X,
            40 => RomanDigit::XL,
            50 => RomanDigit::L,
            90 => RomanDigit::XC,
            100 => RomanDigit::C,
            400 => RomanDigit::CD,
            500 => RomanDigit::D,
            900 => RomanDigit::CM,
            1000 => RomanDigit::M,
            _ => panic!("Invalid Roman Digit"),
        }
    }
}

impl From<u32> for RomanNumber {
    fn from(mut num: u32) -> Self {
        if num == 0 {
            return RomanNumber(vec![RomanDigit::Nulla]);
        }

        let mut roman_digits = vec![];

        let digit_values = vec![1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
        let digit_symbols = vec![M, CM, D, CD, C, XC, L, XL, X, IX, V, IV, I];

        for (i, &value) in digit_values.iter().enumerate() {
            while num >= value {
                roman_digits.push(digit_symbols[i]);
                num -= value;
            }
        }

        let mut res = vec![];

        for digit in roman_digits {
            match digit {
                RomanDigit::Nulla => res.push(RomanDigit::Nulla),
                _ => {
                    let string_digit = format!("{}", digit);
                    for c in string_digit.chars() {
                        match c {
                            'I' | 'V' | 'X' | 'L' | 'C' | 'D' | 'M' => {
                                let new_digit = match c {
                                    'I' => RomanDigit::I,
                                    'V' => RomanDigit::V,
                                    'X' => RomanDigit::X,
                                    'L' => RomanDigit::L,
                                    'C' => RomanDigit::C,
                                    'D' => RomanDigit::D,
                                    'M' => RomanDigit::M,
                                    _ => unreachable!(),
                                };
                                res.push(new_digit);
                            }
                            _ => unreachable!(),
                        }
                    }
                }
            }
        }
        
        RomanNumber(res)
    }
}
