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
     fn from(n:u32)-> Self{
        match n{
            1=> RomanDigit::I,
            _=>RomanDigit::Nulla
        }
    }
} 

impl From<u32> for RomanNumber {

     fn from(n:u32)-> Self{
        let conversions = [
            (1000, vec![RomanDigit::M]),
            (900, vec![RomanDigit::C, RomanDigit::M]),
            (500, vec![RomanDigit::D]),
            (400, vec![RomanDigit::C, RomanDigit::D]),
            (100, vec![RomanDigit::C]),
            (90, vec![RomanDigit::X, RomanDigit::C]),
            (50, vec![RomanDigit::L]),
            (40, vec![RomanDigit::X, RomanDigit::L]),
            (10, vec![RomanDigit::X]),
            (9, vec![RomanDigit::I, RomanDigit::X]),
            (5, vec![RomanDigit::V]),
            (4, vec![RomanDigit::I, RomanDigit::V]),
            (1, vec![RomanDigit::I]),
        ];

        let mut result : Vec<RomanDigit> =  Vec::new();
        let mut i = n;
        for v in conversions.iter(){
            while i >= v.0{
                result.extend(&v.1);
               i= i-v.0;
            }
        }
        RomanNumber(result)
    }
}
