pub use chrono::{Datelike, NaiveDate, Weekday as wd};

pub fn middle_day(year: i32) -> Option<wd> {
    let jours_dans_annee = if NaiveDate::from_ymd_opt(year,2,29).is_some(){
        366
    } else {
        365
    };

    if jours_dans_annee % 2 == 0 {
        return None;
    } else {
        let jours_median_annee = (jours_dans_annee+1)/2;
        let date_median = match NaiveDate::from_yo_opt(year, jours_median_annee as u32){
            Some(d) => d,
            None => panic!("Invalid Date"),
        };
        Some(date_median.weekday())
    }
}
