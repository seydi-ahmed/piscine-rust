use chrono::{Datelike, Weekday};

pub fn middle_day(year: i32) -> Option<Weekday> {
    let days_in_year = if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
        366 // Leap year
    } else {
        365 // Non-leap year
    };

    // Check if the year has an even number of days
    if days_in_year % 2 == 0 {
        return None;
    }

    // Calculate the middle day of the year
    let middle_day = (days_in_year / 2) + 1; // Add 1 to handle 1-based indexing

    // Create a date for the middle day of the year
    let middle_date = chrono::NaiveDate::from_yo(year, middle_day as u32);

    // Get the weekday of the middle day
    Some(middle_date.weekday())
}
