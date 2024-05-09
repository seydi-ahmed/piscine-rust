// use crate::RomanDigit::*;

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
    fn from(value: u32) -> Self {
        if value == 0 {
            return RomanDigit::Nulla;
        } else if value == 1 {
            return RomanDigit::I;
        } else if value == 5 {
            return RomanDigit::V;
        } else if value == 10 {
            return RomanDigit::X;
        } else if value == 50 {
            return RomanDigit::L;
        } else if value == 100 {
            return RomanDigit::C;
        } else if value == 500 {
            return RomanDigit::D;
        } else if value == 1000 {
            return RomanDigit::M;
        } else {
            return RomanDigit::Nulla;
        }
    }
}

impl From<u32> for RomanNumber {
    fn from(value: u32) -> Self {
        let roman_str = int_to_roman(value);
        let mut v = Vec::new();
        if roman_str.is_empty() {
            v.push(RomanDigit::Nulla);
        }

        for s in roman_str.chars() {
            v.push(match s {
                'I' => RomanDigit::I,
                'V' => RomanDigit::V,
                'X' => RomanDigit::X,
                'L' => RomanDigit::L,
                'C' => RomanDigit::C,
                'D' => RomanDigit::D,
                'M' => RomanDigit::M,
                _ => RomanDigit::Nulla,
            });
        }

        RomanNumber(v)
    }
}

fn int_to_roman(value: u32) -> String {
    let m = vec!["", "M", "MM", "MMM"];
    let c = vec!["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM "];
    let x = vec!["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];
    let i = vec!["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];

    let mut roman = String::new();
    roman += m[(value / 1000) as usize];
    roman += c[((value % 1000) / 100) as usize];
    roman += x[((value % 100) / 10) as usize];
    roman += i[(value % 10) as usize];

    roman
}
