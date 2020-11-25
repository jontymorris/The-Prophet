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
        end_date: parse_date("2020-10-01".to_string()),
    };

    println!("Running simulation...");
    let result = perform_simulation(stocks, &config);

    print_results(&result, &config);
    save_trades(&result.trades);
}

fn print_results(result: &Portfolio, config: &Config) {
    println!("\nFinal balance: ${:.3}", result.balance);

    // total and annual returns
    let total_returns = get_percent_change(result.balance, config.balance);
    let duration = config.end_date - config.start_date;
    let years = duration.num_days() as f64 / 365.0;
    let annual_returns = total_returns / years;

    println!("\nTotal returns: {:.3}%", total_returns);
    println!("Annual returns: {:.3}%", annual_returns);

    // W/L ratio
    let winners: Vec<&Trade> = result
        .trades
        .iter()
        .filter(|&x| x.sell_price > x.buy_price)
        .collect();
    let losers: Vec<&Trade> = result
        .trades
        .iter()
        .filter(|&x| x.sell_price <= x.buy_price)
        .collect();
    let win_loss_ratio = winners.len() as f64 / losers.len() as f64;

    println!("\n{} trades made", result.trades.len());
    println!("{:.3} W/L ratio", win_loss_ratio);

    // win % and avg % amount
    let (win_percent, avg_winnings) = get_chances(winners, result);
    println!(
        "\n{:.3}% succeeded, avg {:.3}% profit",
        win_percent, avg_winnings
    );

    // loss % and avg % amount
    let (loss_percent, avg_losings) = get_chances(losers, result);
    println!("{:.3}% failed, avg {:.3}% loss", loss_percent, avg_losings);
}

fn get_chances(values: Vec<&Trade>, result: &Portfolio) -> (f64, f64) {
    let probability = (values.len() as f64 / result.trades.len() as f64) * 100.0;
    let sum: f64 = values
        .iter()
        .map(|x| get_percent_change(x.sell_price, x.buy_price))
        .sum();

    let avg = sum / values.len() as f64;

    return (probability, avg);
}
