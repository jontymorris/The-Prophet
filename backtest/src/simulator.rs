use std::collections::HashMap;
use chrono::{NaiveDate, Duration};
use serde::Serialize;
use stocks_core::types::*;
use stocks_core::util::*;
use stocks_core::stats::*;
use stocks_core::trading::*;

pub struct Config {
    pub balance: f32,
    pub buy_amount: f32,
    pub sell_loss_percent: f32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub step_size: usize,
}

pub struct Portfolio {
    pub balance: f32,
    pub investments: HashMap<String, Investment>,
    pub trades: Vec<Trade>
}

pub struct Investment {
    pub price: f32,
    pub quantity: f32
}

#[derive(Serialize)]
pub struct Trade {
    pub symbol: String,
    pub price: f32,
    pub date: String,
    pub was_buying: bool,
}

pub fn perform_simulation(stocks: Vec<Stock>, bounds: HashMap<String, Bound>, config: &Config) -> Portfolio {
    let mut portfolio = Portfolio {
        balance: config.balance,
        investments: HashMap::new(),
        trades: vec![]
    };

    let mut trading_date = config.start_date;
    let today = get_today();
    let interval = Duration::days(1);

    // keep looping until past end-date
    while !is_past_date(trading_date, config.end_date) {
        // check the stocks each day
        for stock in &stocks {
            let bound: &Bound = &bounds[&stock.symbol];
            check_stock(stock, bound, trading_date, &mut portfolio, config.buy_amount);
        }

        trading_date += interval;
    }

    // now sell the remaining investments
    while !is_past_date(trading_date, today) && portfolio.investments.len() > 0 {
        for stock in &stocks {
            if portfolio.investments.contains_key(&stock.symbol) {
                let bound: &Bound = &bounds[&stock.symbol];
                check_stock(stock, bound, trading_date, &mut portfolio, config.buy_amount);
            }
        }

        trading_date += interval;
    }

    return portfolio;
}

fn check_stock(stock: &Stock, bound: &Bound, current_date: NaiveDate, portfolio: &mut Portfolio, buy_amount: f32) {
    // skip pre-listed stocks
    if !has_been_listed_yet(stock, current_date) {
        return;
    }

    // check we have closing data for this day
    let closing = get_latest_close(stock, current_date);
    if closing.is_none() {
        return;
    }

    // analyze the buying and selling
    if portfolio.investments.contains_key(&stock.symbol) {
        analyze_selling(stock, bound, closing.unwrap(), portfolio);
    } else {
        analyze_buying(stock, bound, closing.unwrap(), portfolio, buy_amount);
    }
}

fn analyze_buying(stock: &Stock, bound: &Bound, closing: Close, portfolio: &mut Portfolio, buy_amount: f32) {
    if should_buy(&closing, bound) {
        // ignore if we don't have the budget
        if portfolio.balance < buy_amount {
            return;
        }

        let quantity = buy_amount / closing.value;
        println!("> Buying {} at ${}", stock.symbol, closing.value);

        let new_investment = Investment {
            price: closing.value,
            quantity: quantity,
        };

        let new_trade = Trade {
            price: closing.value,
            date: closing.date.clone(),
            symbol: stock.symbol.clone(),
            was_buying: true
        };

        portfolio.balance -= buy_amount;
        portfolio.trades.push(new_trade);
        portfolio.investments.insert(stock.symbol.clone(), new_investment);
    }
}

fn analyze_selling(stock: &Stock, bound: &Bound, closing: Close, portfolio: &mut Portfolio) {
    let investment = &portfolio.investments[&stock.symbol];

    if should_sell(&closing, bound, investment.price) {
        println!("> Selling {} at ${}", stock.symbol, closing.value);
        let new_amount = closing.value * investment.quantity;
        
        let new_trade = Trade {
            price: closing.value,
            symbol: stock.symbol.clone(),
            date: closing.date.clone(),
            was_buying: false
        };

        portfolio.trades.push(new_trade);
        portfolio.balance += new_amount;
        portfolio.investments.remove_entry(&stock.symbol).unwrap();
    }
}

fn get_latest_close(stock: &Stock, current_date: NaiveDate) -> Option<Close> {
    let format = "%Y-%m-%d";
    let date_value = current_date.format(format).to_string();

    for (index, candle) in stock.history.iter().enumerate() {
        if candle.date.eq(&date_value) {
            // no change on first candle
            if index == 0 {
                return None;
            }

            let previous = &stock.history[index-1];

            return Some(Close {
                value: candle.close,
                percent_change: get_change(candle.close, previous.close),
                date: candle.date.clone()
            });
        }
    }

    return None;
}

fn has_been_listed_yet(stock: &Stock, current_date: NaiveDate) -> bool {
    let listing_date = parse_date(stock.listing_date.clone());
    is_past_date(current_date, listing_date)
}