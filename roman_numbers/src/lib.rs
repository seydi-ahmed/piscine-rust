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
    fn from(num: u32) -> Self {
        match num {
            1 => Self::I,
            5 => Self::V,
            10 => Self::X,
            50 => Self::L,
            100 => Self::C,
            500 => Self::D,
            1000 => Self::M,
            _ => Self::Nulla,
        }
    }
}


impl From<u32> for RomanNumber {
    fn from(mut num: u32) -> Self {
        let mut digits: Vec<RomanDigit> = Vec::new();
        let numerals = vec![1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
        let symbols = vec!["M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"];
        let mut index = 0;
        if num == 0 {
            digits.push(RomanDigit::Nulla);
            return Self(digits);
        }
        while num > 0 {
            let mut quotient = num / numerals[index];
            num %= numerals[index];
            while quotient > 0 {
                digits.extend(symbols[index].chars().map(|c| match c {
                    'M' => RomanDigit::M,
                    'D' => RomanDigit::D,
                    'C' => RomanDigit::C,
                    'L' => RomanDigit::L,
                    'X' => RomanDigit::X,
                    'V' => RomanDigit::V,
                    'I' => RomanDigit::I,
                    _ => RomanDigit::Nulla,
                }));
                quotient -= 1;
            }
            index += 1;
        }

        Self(digits)
    }
}   