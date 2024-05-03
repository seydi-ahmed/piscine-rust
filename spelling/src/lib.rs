Bro nkm
Santeu Al Hamdoulilah
Diégalou dh
Dafa am louma done xol
Boy défar na cube bi
D’accord
Boy nagn am lépeu après gnou diéma comprendre
Push ko dh
Push nako
D'accord fils
Okay
Boy 2 last exos yi
Lib.rs bi
Neuy gaaw way
rust
use edit_distance::levenshtein;

pub fn expected_variable(source: &str, target: &str) -> Option<String> {

    if source.contains(" ") || source.contains("-") || source.chars().all(|ch| ch.is_ascii_lowercase()) || source.chars().all(|ch| ch.is_ascii_uppercase()){
        return  None;
    }
    let res = levenshtein(source.to_lowercase().as_str(), target.to_lowercase().as_str());
    
    println!("res {}", res);
    if res < target.len() / 2{ 
        let percentage = (100 - (res as f64 / target.len() as f64 * 100.0) as u8) as u8;
        
        return Some(format!("{}%", percentage));
    }
    None
}



lib.rs


#[must_use]
pub fn levenshtein(a: &str, b: &str) -> usize {
    let mut result = 0;

    /* Shortcut optimizations / degenerate cases. */
    if a == b {
        return result;
    }

    let length_a = a.chars().count();
    let length_b = b.chars().count();

    if length_a == 0 {
        return length_b;
    }

    if length_b == 0 {
        return length_a;
    }

    /* Initialize the vector.
     *
     * This is why it’s fast, normally a matrix is used,
     * here we use a single vector. */
    let mut cache: Vec<usize> = (1..).take(length_a).collect();
    let mut distance_a;
    let mut distance_b;

    /* Loop. */
    for (index_b, code_b) in b.chars().enumerate() {
        result = index_b;
        distance_a = index_b;

        for (index_a, c…

[package]
name = "expected_variable"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
case = "1.0.0"
edit_distance = { path = "../edit_distance"}

cargo.toml
Diadieuf bro
Surtout importation edit distance bi
Boy séne mobs bi meunou ma si lou beuri
Teudina

pub fn spell(n: u64) -> String {
    let ones = ["", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let teens = ["ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"];
    let tens = ["", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"];
    let thousands = ["thousand", "million"];
   
    if n == 0{
        return "zero".to_string()
    } else if n < 10 {
        return ones[n as usize].to_string();
    } else if n < 20 {
        return teens[(n - 10) as usize].to_string();
    } else if n < 100 {
        let tens_index = (n / 10) as usize;
        let ones_index = (n % 10) as usize;
        if ones_index != 0 {
            return format!("{}-{}", tens[tens_index], ones[ones_index]);
        }
        return format!("{}", tens[tens_index]);
    } else if n < 1000 {
        let hundreds = n / 100;
        let remainder = n % 100;
        if remainder != 0{
            return format!("{} hundred {}", ones[hundreds as usize], spell(remainder));
        }
        return format!("{} hundred", ones[hundreds as usize])
    } else {
        for (index, &thousand) in thousands.iter().enumerate().rev() {
            let base = 10u64.pow((index as u32 + 1) * 3);
            if n >= base {
                let prefix = spell(n / base);
                let suffix = if n % base == 0 { "".to_string() } else { format!(" {}", spell(n % base)) };
                return format!("{} {}{}", prefix, thousand, suffix);
            }
        }
        unreachable!(); 
    }
}



// pub fn spell(n: u64) -> String {
//     if n == 0 {
//         return "zero".to_string();
//     }
//     let mut result = String::new();
//     if n >= 1_000_000 {
//         let millions = n / 1_000_000;
//         result.push_str(&spell_less_than_1000(millions));
//         result.push_str(" million");
//         if n % 1_000_000 != 0 {
//             result.push(' ');
//         }
//     }
//     if n >= 1_000_000 {
//         let millions = (n / 1_000_000) % 1000;
//         result.push_str(&spell_less_than_1000(millions));
//         result.push_str(" million");
//         if n % 1_000_000 != 0 {
//             result.push(' ');
//         }
//     }
//     if n >= 1_000 {
//         let thousands = (n / 1_000) % 1000;
//         result.push_str(&spell_less_than_1000(thousands));
//         result.push_str(" thousand");
//         if n % 1_000 != 0 {
//             result.push(' ');
//         }
//     }
//     if n >= 100 {
//         let hundreds = (n / 100) % 10;
//         result.push_str(&spell_hundreds(hundreds));
//         result.push_str(" hundred");
//         if n % 100 != 0 {
//             result.push(' ');
//         }
//     }
//     let less_than_hundred = n % 100;
//     if less_than_hundred > 0 {
//         if less_than_hundred < 20 {
//             result.push_str(&spell_less_than_20(less_than_hundred));
//         } else {
//             let tens = (less_than_hundred / 10) * 10;
//             result.push_str(&spell_tens(tens));
//             let units = less_than_hundred % 10;
//             if units > 0 {
//                 result.push('-');
//                 result.push_str(&spell_units(units));
//             }
//         }
//     }

//     result
// }
// fn spell_less_than_1000(n: u64) -> String {
//     if n < 100 {
//         spell_less_than_100(n)
//     } else {
//         let hundreds = n / 100;
//         let remaining = n % 100;
//         let mut result = spell_hundreds(hundreds);
//         if remaining > 0 {
//             result.push(' ');
//             result.push_str(&spell_less_than_100(remaining));
//         }
//         result
//     }
// }
// fn spell_less_than_100(n: u64) -> String {
//     if n < 20 {
//         spell_less_than_20(n)
//     } else {
//         let tens = (n / 10) * 10;
//         let units = n % 10;
//         let mut result = spell_tens(tens);
//         if units > 0 {
//             result.push('-');
//             result.push_str(&spell_units(units));
//         }
//         result
//     }
// }
// fn spell_less_than_20(n: u64) -> String {
//     match n {
//         0 => "zero".to_string(),
//         1 => "one".to_string(),
//         2 => "two".to_string(),
//         3 => "three".to_string(),
//         4 => "four".to_string(),
//         5 => "five".to_string(),
//         6 => "six".to_string(),
//         7 => "seven".to_string(),
//         8 => "eight".to_string(),
//         9 => "nine".to_string(),
//         10 => "ten".to_string(),
//         11 => "eleven".to_string(),
//         12 => "twelve".to_string(),
//         13 => "thirteen".to_string(),
//         14 => "fourteen".to_string(),
//         15 => "fifteen".to_string(),
//         16 => "sixteen".to_string(),
//         17 => "seventeen".to_string(),
//         18 => "eighteen".to_string(),
//         19 => "nineteen".to_string(),
//         _ => unreachable!(),
//     }
// }
// fn spell_tens(n: u64) -> String {
//     match n {
//         20 => "twenty".to_string(),
//         30 => "thirty".to_string(),
//         40 => "forty".to_string(),
//         50 => "fifty".to_string(),
//         60 => "sixty".to_string(),
//         70 => "seventy".to_string(),
//         80 => "eighty".to_string(),
//         90 => "ninety".to_string(),
//         _ => unreachable!(),
//     }
// }
// fn spell_units(n: u64) -> String {
//     match n {
//         1 => "one".to_string(),
//         2 => "two".to_string(),
//         3 => "three".to_string(),
//         4 => "four".to_string(),
//         5 => "five".to_string(),
//         6 => "six".to_string(),
//         7 => "seven".to_string(),
//         8 => "eight".to_string(),
//         9 => "nine".to_string(),
//         _ => unreachable!(),
//     }
// }
// fn spell_hundreds(n: u64) -> String {
//     match n {
//         1 => "one".to_string(),
//         2 => "two".to_string(),
//         3 => "three".to_string(),
//         4 => "four".to_string(),
//         5 => "five".to_string(),
//         6 => "six".to_string(),
//         7 => "seven".to_string(),
//         8 => "eight".to_string(),
//         9 => "nine".to_string(),
//         _ => unreachable!(),
//     }
// }