pub use chrono::{Datelike, NaiveDate, Weekday};

pub fn middle_day(year: i32) -> Option<Weekday> {
    let days_in_year = if NaiveDate::from_ymd_opt(year,2,29).is_some(){
        366
    } else {
        365
    };

    if days_in_year % 2 == 0 {
        return None;
    } else {
        let middleDay = (days_in_year+1)/2;
        let middleDate = match NaiveDate::from_ymd_opt(year, middleDay as u32){
            Some(d) => d,
            None => panic!("Invalid Date"),
        };
        Some(middleDate.weekday())
    }
}
