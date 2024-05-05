pub fn spell(n: u64) -> String {
    let ones = ["", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let teens = ["ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"];
    let tens = ["", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"];
    let thousands = ["thousand", "million"];

    // ZERO
    if n == 0 {
        return "zero".to_string();
    }

    // UNITÉ
    if n < 10 {
        return ones[n as usize].to_string();
    }

    // DIZAINE
    if n < 20 {
        return teens[(n - 10) as usize].to_string();
    }

    // CENTAINE
    if n < 100 {
        let unite : usize = n as usize % 10;
        let dizaine : usize = n as usize / 10;
        
        if unite != 0{
            return format!("{}-{}", tens[dizaine].to_string(), ones[unite].to_string());
        } else {
            return format!("{}", tens[dizaine].to_string());   
        }
    }

    // MILLIÉME
    if n < 1000 {
        let hundreds = n / 100;
        let remainder = n % 100;

        if remainder != 0{
            return format!("{} hundred {}", ones[hundreds as usize], spell(remainder));
        }

        return format!("{} hundred", ones[hundreds as usize]);
    }

    
    // MILLION
    for (index, &thousand) in thousands.iter().enumerate().rev() {
        let base = 10u64.pow((index as u32 + 1) * 3);
        if n >= base {
            let prefix = spell(n / base);
            let suffix = if n % base == 0 {
                "".to_string()
            } else {
                format!(" {}", spell(n % base))
            };
            return format!("{} {}{}", prefix, thousand, suffix);
        }
    }
    unreachable!(); 
}


fn main() {
    println!("{}", spell(0));
    println!("{}", spell(8));
    println!("{}", spell(18));
    println!("{}", spell(48));

    println!("{}", spell(348));
    println!("{}", spell(9996));
}