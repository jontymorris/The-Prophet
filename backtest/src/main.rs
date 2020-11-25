mod data;
mod simulator;

use data::{get_all_stocks, save_trades};
use simulator::*;
use stocks_core::dates::parse_date;
use stocks_core::stats::*;

fn main() {
    let stocks = get_all_stocks("us".into());

    let config = Config {
        balance: 10000.0,
        buy_amount: 2500.0,
        sell_loss_percent: 7.0,
        sell_gain_percent: 5.0,
        days_to_go_back: 150,
        start_date: parse_date("2016-03-01".to_string()),
        end_date: parse_date("2019-03-01".to_string()),
    };

    println!("Running simulation...");
    let result = perform_simulation(stocks, &config);

    println!("\nFinal balance: ${}", result.balance);
    println!("{} trades made", result.trades.len());

    let total_returns = get_percent_change(result.balance, config.balance);
    let duration = config.end_date - config.start_date;
    let years = duration.num_days() as f64 / 365.0;
    let annual_returns = total_returns / years;

    println!("\nTotal returns: {}%", total_returns);
    println!("Annual returns: {}%", annual_returns);

    save_trades(&result.trades);
}
