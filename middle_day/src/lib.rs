use chrono::{Weekday, NaiveDate, Datelike};

pub fn middle_day(year: i32) -> Option<Weekday> {
    let start_date = NaiveDate::from_yo_opt(year, 1)?.and_hms_opt(0, 0, 0)?;
    let end_date = NaiveDate::from_yo_opt(year + 1, 1)?.and_hms_opt(0, 0, 0)?;

    let days_in_year = (end_date - start_date).num_days();

    if days_in_year % 2 == 0 {
        None
    } else {
        let middle_date = start_date + chrono::Duration::days(days_in_year / 2);
        Some(middle_date.weekday())
    }
}
