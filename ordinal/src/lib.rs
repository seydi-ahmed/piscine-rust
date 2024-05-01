pub fn num_to_ordinal(x: u32) -> String {
    // Get the last digit of the number
    let last_digit = x % 10;
    // Get the last two digits of the number
    let last_two_digits = x % 100;

    // Determine the ordinal suffix based on the rules
    let suffix = match last_digit {
        1 if last_two_digits != 11 => "st",
        2 if last_two_digits != 12 => "nd",
        3 if last_two_digits != 13 => "rd",
        _ => "th",
    };

    // Return the number with the appropriate ordinal suffix
    format!("{}{}", x, suffix)
}
