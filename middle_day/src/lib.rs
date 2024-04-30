use chrono::{Datelike, NaiveDate, Weekday};

pub fn middle_day(year: i32) -> Option<Weekday> {
    // Vérifier si l'année est bissextile
    let is_leap_year = NaiveDate::from_ymd_opt(year, 2, 29).is_some();

    // Si l'année est bissextile ou non
    let days_in_year = if is_leap_year { 366 } else { 365 };

    // Si le nombre de jours dans l'année est pair, il n'y a pas de jour médian
    if days_in_year % 2 == 0 {
        return None;
    }

    // Calculer la date médiane de l'année
    let middle_date = NaiveDate::from_ymd_opt(year, 1, 1)?.checked_add_signed(chrono::Duration::days((days_in_year / 2) as i64));

    // Extraire le jour de la semaine de la date médiane, s'il existe
    middle_date.map(|date| date.weekday())
}
