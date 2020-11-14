use std::fs::{write, read_to_string};
use stocks_core::types::*;
use stocks_core::util::*;
use super::simulator::Trade;

pub fn get_all_stocks() -> Vec<Stock> {
    let json = read_to_string("assets/stocks.json").unwrap();

    // load the stock listings from json
    let result = serde_json::from_str(json.as_str());
    let mut stocks: Vec<Stock> = result.unwrap();

    // now get their histories
    for i in 0..stocks.len() {
        let stock = stocks.get_mut(i).unwrap();
        stock.history = get_stock_history(&stock);
    }

    return stocks;
}

fn get_stock_history(stock: &Stock) -> Vec<Candle> {
    // read the CSV contents
    let symbol = stock.symbol.to_ascii_uppercase();
    let path = format!("assets/history/{}.csv", symbol);
    let contents = read_to_string(path).unwrap();

    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b',')
        .from_reader(contents.as_bytes());

    let mut history: Vec<Candle> = vec![];
    let listing_date = parse_date(stock.listing_date.clone());

    // now parse each CSV record
    for result in reader.deserialize() {
        let candle: Candle = result.unwrap();
        let date = parse_date(candle.date.clone());

        // ensure it is past the listing date
        if is_past_date(date, listing_date) {
            history.push(candle);
        }
    }

    return history;
}

pub fn save_trades(trades: &Vec<Trade>) {
    let json_contents = serde_json::to_string_pretty(trades).unwrap();
    write("assets/trades.json", json_contents).unwrap();
}
