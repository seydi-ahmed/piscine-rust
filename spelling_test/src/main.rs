pub fn spell(n: u64) -> String {
    let ones = ["", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let teens = ["ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"];
    let tens = ["", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"];
    let thousands = ["thousand", "million"];

    // zero
    if n == 0 {
        return "zero".to_string();
    }

    // unite
    if n < 10 {
        return ones[n as usize].to_string();
    }

    // dizaine
    if n < 20 {
        return teens[(n - 10) as usize].to_string();
    }

    // centaine
    if n < 100 {
        let quotient : u64 = n/10;
        let reste : u64 = n%10;

        if reste != 0{
            return format!("{}-{}", tens[quotient as usize], ones[reste as usize]);
        } else {
            return format!("{}", tens[quotient as usize]);
        }
    }

    // millier && million
    if n < 1000{
        let quotient : u64 = n/100;
        let reste : u64 = n%100;
        if reste == 0 {
            return format!("{} hundred", spell(quotient));
        }else{
            return format!("{} hundred {}", spell(quotient), spell(reste));
        }
    } else {
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
}

fn main() {
    println!("{}: {}", 0, spell(0));
    println!("{}: {}", 8, spell(8));
    println!("{}: {}", 10, spell(10));
    println!("{}: {}", 20, spell(20));
    println!("{}: {}", 48,  spell(48));
    println!("{}: {}", 100, spell(100));
    println!("{}: {}", 348, spell(348));
    println!("{}: {}", 9996, spell(9996));
}