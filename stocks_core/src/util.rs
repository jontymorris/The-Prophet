use chrono::{Duration, NaiveDate};
use chrono::offset::Local;

pub fn parse_date(value: String) -> NaiveDate {
    let format = "%Y-%m-%d";
    NaiveDate::parse_from_str(value.as_str(), format).unwrap()
}

pub fn is_past_date(current: NaiveDate, compare: NaiveDate) -> bool {
    let difference = current.signed_duration_since(compare);
    let one_day_difference = Duration::days(1);

    return difference > one_day_difference;
}

pub fn get_today() -> NaiveDate {
    let now = Local::now();
    let format = "%Y-%m-%d";

    return parse_date(now.format(format).to_string());
}
