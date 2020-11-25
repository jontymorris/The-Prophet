use chrono::{Duration, NaiveDate};
use indicatif::ProgressBar;
use serde::Serialize;
use std::collections::HashMap;
use stocks_core::dates::*;
use stocks_core::trading::*;
use stocks_core::types::*;

pub struct Config {
    pub balance: f64,
    pub buy_amount: f64,
    pub sell_loss_percent: f64,
    pub sell_gain_percent: f64,
    pub days_to_go_back: i64,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

pub struct Portfolio {
    pub balance: f64,
    pub investments: HashMap<String, Investment>,
    pub trades: Vec<Trade>,
}

pub struct Investment {
    pub price: f64,
    pub quantity: f64,
}

#[derive(Serialize)]
pub struct Trade {
    pub symbol: String,
    pub price: f64,
    pub quantity: f64,
    pub date: String,
    pub was_buying: bool,
}

pub fn perform_simulation(stocks: Vec<Stock>, config: &Config) -> Portfolio {
    let mut portfolio = Portfolio {
        balance: config.balance,
        investments: HashMap::new(),
        trades: vec![],
    };

    let mut trading_date = config.start_date;
    let today = get_today();
    let interval = Duration::days(1);

    let days = (config.end_date - config.start_date).num_days();
    let progress = ProgressBar::new(days as u64);

    // keep looping until past end-date
    while !is_past_date(trading_date, config.end_date) {
        // check the stocks each day
        for stock in &stocks {
            check_stock(stock, trading_date, &mut portfolio, &config);
        }

        trading_date += interval;
        progress.inc(1);
    }

    progress.finish();

    cleanup_portfolio(&stocks, &mut portfolio, &mut trading_date, today, interval);

    return portfolio;
}

fn cleanup_portfolio(
    stocks: &Vec<Stock>,
    portfolio: &mut Portfolio,
    trading_date: &mut NaiveDate,
    final_date: NaiveDate,
    interval: Duration,
) {
    // add remaining investments back to the portfolio at last market price
    for (symbol, investment) in portfolio.investments.iter() {
        let stock = stocks.iter().find(|&x| x.symbol.eq(symbol)).unwrap();

        // try to find the last price for this stock
        let mut closes = stock.history.iter().map(|x| x.close).collect();
        while !is_past_date(*trading_date, final_date) {
            match get_recent_closes(stock, *trading_date, 3) {
                Some(values) => {
                    closes = values;
                    break;
                }
                None => {
                    *trading_date += interval;
                }
            }
        }

        let last_price = *closes.last().unwrap();
        let amount = investment.quantity * last_price;

        let new_trade = Trade {
            date: trading_date.format("%Y-%m-%d").to_string(),
            price: last_price,
            quantity: investment.quantity,
            symbol: symbol.clone(),
            was_buying: false,
        };

        portfolio.balance += amount;
        portfolio.trades.push(new_trade);
    }
}

fn check_stock(stock: &Stock, current_date: NaiveDate, portfolio: &mut Portfolio, config: &Config) {
    // skip pre-listed stocks
    if !has_been_listed_yet(stock, current_date) {
        return;
    }

    // check we have closing data for this day
    let recent_closes = get_recent_closes(stock, current_date, config.days_to_go_back);
    if recent_closes.is_none() {
        return;
    }

    // analyze the buying and selling
    if portfolio.investments.contains_key(&stock.symbol) {
        analyze_selling(
            stock,
            &recent_closes.unwrap(),
            &current_date,
            portfolio,
            config,
        );
    } else {
        analyze_buying(
            stock,
            &recent_closes.unwrap(),
            &current_date,
            portfolio,
            config,
        );
    }
}

fn analyze_buying(
    stock: &Stock,
    recent_closes: &Vec<f64>,
    current_date: &NaiveDate,
    portfolio: &mut Portfolio,
    config: &Config,
) {
    if should_buy(&recent_closes) {
        // don't re-enter recent trades
        if has_sold_recently(stock, current_date, portfolio) {
            return;
        }

        let buy_amount = get_risk_factored_amount(config.buy_amount, recent_closes);
        let last_close_price = *recent_closes.last().unwrap();

        // check we have the balance
        if buy_amount == 0.0 || portfolio.balance < buy_amount {
            return;
        }

        let quantity = (buy_amount * 0.995) / last_close_price;

        let new_investment = Investment {
            price: last_close_price,
            quantity: quantity,
        };

        let new_trade = Trade {
            price: last_close_price,
            quantity: quantity,
            date: current_date.format("%Y-%m-%d").to_string(),
            symbol: stock.symbol.clone(),
            was_buying: true,
        };

        portfolio.balance -= buy_amount;
        portfolio.trades.push(new_trade);
        portfolio
            .investments
            .insert(stock.symbol.clone(), new_investment);
    }
}

fn analyze_selling(
    stock: &Stock,
    recent_closes: &Vec<f64>,
    current_date: &NaiveDate,
    portfolio: &mut Portfolio,
    config: &Config,
) {
    let investment = &portfolio.investments[&stock.symbol];
    let latest_close = *recent_closes.last().unwrap();
    let date_format = current_date.format("%Y-%m-%d").to_string();

    if should_sell(
        &recent_closes,
        investment.price,
        config.sell_loss_percent,
        config.sell_gain_percent,
    ) {
        let new_amount = latest_close * investment.quantity;

        let new_trade = Trade {
            price: latest_close,
            quantity: investment.quantity,
            symbol: stock.symbol.clone(),
            date: date_format,
            was_buying: false,
        };

        portfolio.trades.push(new_trade);
        portfolio.balance += new_amount * 0.995;
        portfolio.investments.remove_entry(&stock.symbol).unwrap();
    }
}

fn has_been_listed_yet(stock: &Stock, current_date: NaiveDate) -> bool {
    let listing_date = parse_date(stock.listing_date.clone());
    is_past_date(current_date, listing_date)
}

fn has_sold_recently(stock: &Stock, today: &NaiveDate, portfolio: &Portfolio) -> bool {
    let trades: Vec<&Trade> = portfolio
        .trades
        .iter()
        .filter(|&x| x.symbol.eq(&stock.symbol))
        .collect();

    let last_trade = trades.last();
    if last_trade.is_some() {
        let trade_date = parse_date(last_trade.unwrap().date.clone());
        let duration = *today - trade_date;
        return duration.num_days() < 35;
    }

    return false;
}
