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
            0 => RomanDigit::Nulla,
            1 => RomanDigit::I,
            5 => RomanDigit::V,
            10 => RomanDigit::X,
            50 => RomanDigit::L,
            100 => RomanDigit::C,
            500 => RomanDigit::D,
            1000 => RomanDigit::M,
            _ => panic!("Invalid Roman Digit"),
        }
    }
}

impl From<u32> for RomanNumber {
    fn from(mut num: u32) -> Self {
        let mut roman_digits = vec![];

        let digit_values = vec![1000, 500, 100, 50, 10, 5, 1];
        let digit_symbols = vec![M, D, C, L, X, V, I];

        for (i, &value) in digit_values.iter().enumerate() {
            while num >= value {
                roman_digits.push(digit_symbols[i]);
                num -= value;
            }
            if i % 2 == 0 && i < 6 {
                let next_value = digit_values[i + 2];
                let next_symbol = digit_symbols[i + 2];
                if num >= value - next_value {
                    roman_digits.push(next_symbol);
                    roman_digits.push(digit_symbols[i]);
                    num -= value - next_value;
                }
            } else if i % 2 == 1 && i < 5 {
                let next_value = digit_values[i + 1];
                let next_symbol = digit_symbols[i + 1];
                if num >= value - next_value {
                    roman_digits.push(next_symbol);
                    roman_digits.push(digit_symbols[i]);
                    num -= value - next_value;
                }
            }
        }

        if roman_digits.is_empty() {
            roman_digits.push(Nulla);
        }
        RomanNumber(roman_digits)
    }
}
