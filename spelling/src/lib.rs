pub fn spell(n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }

    let mut result = String::new();

    // Traitement des milliards
    if n >= 1_000_000_000 {
        let billions = n / 1_000_000_000;
        result.push_str(&spell_less_than_1000(billions));
        result.push_str(" billion");
        if n % 1_000_000_000 != 0 {
            result.push(' ');
        }
    }

    // Traitement des millions
    if n >= 1_000_000 {
        let millions = (n / 1_000_000) % 1000;
        result.push_str(&spell_less_than_1000(millions));
        result.push_str(" million");
        if n % 1_000_000 != 0 {
            result.push(' ');
        }
    }

    // Traitement des milliers
    if n >= 1_000 {
        let thousands = (n / 1_000) % 1000;
        result.push_str(&spell_less_than_1000(thousands));
        result.push_str(" thousand");
        if n % 1_000 != 0 {
            result.push(' ');
        }
    }

    // Traitement des centaines
    if n >= 100 {
        let hundreds = (n / 100) % 10;
        result.push_str(&spell_hundreds(hundreds));
        result.push_str(" hundred");
        if n % 100 != 0 {
            result.push(' ');
        }
    }

    // Traitement des dizaines et des unités
    let less_than_hundred = n % 100;
    if less_than_hundred > 0 {
        if n >= 100 {
            result.push_str("and ");
        }
        if less_than_hundred < 20 {
            result.push_str(&spell_less_than_20(less_than_hundred));
        } else {
            let tens = (less_than_hundred / 10) * 10;
            result.push_str(&spell_tens(tens));
            let units = less_than_hundred % 10;
            if units > 0 {
                result.push('-');
                result.push_str(&spell_units(units));
            }
        }
    }

    result
}

// Fonction auxiliaire pour les nombres inférieurs à 1000
fn spell_less_than_1000(n: u64) -> String {
    if n < 100 {
        spell_less_than_100(n)
    } else {
        let hundreds = n / 100;
        let remaining = n % 100;
        let mut result = spell_hundreds(hundreds);
        if remaining > 0 {
            result.push(' ');
            result.push_str(&spell_less_than_100(remaining));
        }
        result
    }
}

// Fonction auxiliaire pour les nombres inférieurs à 100
fn spell_less_than_100(n: u64) -> String {
    if n < 20 {
        spell_less_than_20(n)
    } else {
        let tens = (n / 10) * 10;
        let units = n % 10;
        let mut result = spell_tens(tens);
        if units > 0 {
            result.push('-');
            result.push_str(&spell_units(units));
        }
        result
    }
}

// Fonction auxiliaire pour les nombres inférieurs à 20
fn spell_less_than_20(n: u64) -> String {
    match n {
        0 => "zero".to_string(),
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        10 => "ten".to_string(),
        11 => "eleven".to_string(),
        12 => "twelve".to_string(),
        13 => "thirteen".to_string(),
        14 => "fourteen".to_string(),
        15 => "fifteen".to_string(),
        16 => "sixteen".to_string(),
        17 => "seventeen".to_string(),
        18 => "eighteen".to_string(),
        19 => "nineteen".to_string(),
        _ => unreachable!(),
    }
}

// Fonction auxiliaire pour les dizaines
fn spell_tens(n: u64) -> String {
    match n {
        20 => "twenty".to_string(),
        30 => "thirty".to_string(),
        40 => "forty".to_string(),
        50 => "fifty".to_string(),
        60 => "sixty".to_string(),
        70 => "seventy".to_string(),
        80 => "eighty".to_string(),
        90 => "ninety".to_string(),
        _ => unreachable!(),
    }
}

// Fonction auxiliaire pour les unités
fn spell_units(n: u64) -> String {
    match n {
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        _ => unreachable!(),
    }
}

// Fonction auxiliaire pour les centaines
fn spell_hundreds(n: u64) -> String {
    match n {
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        _ => unreachable!(),
    }
}
