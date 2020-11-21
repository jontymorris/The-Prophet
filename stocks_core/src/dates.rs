use chrono::offset::Local;
use chrono::{Duration, NaiveDate};
use super::types::Stock;

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

pub fn get_recent_closes(stock: &Stock, date: NaiveDate, days_to_go_back: i64) -> Option<Vec<f64>> {
    let date_format = date.format("%Y-%m-%d").to_string();
    let required_length = (days_to_go_back - 1) as usize;

    for (index, candle) in stock.history.iter().enumerate() {
        // find the latest candle
        if !candle.date.eq(&date_format) {
            continue;
        }

        // do we have enough prior candles?
        if index < required_length {
            return None;
        }

        // now add recent closes to vector and return
        let start_index = index - required_length;
        let mut recent_closes: Vec<f64> = Vec::with_capacity(days_to_go_back as usize);
        for i in start_index..index+1 {
            recent_closes.push(stock.history[i].close);
        }

        return Some(recent_closes);
    }

    return None;
}
