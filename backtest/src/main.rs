mod data;
mod simulator;

use data::{get_all_stocks, save_trades};
use simulator::*;
use stocks_core::stats::*;
use stocks_core::util::parse_date;

fn main() {
    let step_size = 3;

    println!("Loading stocks...");
    let stocks = get_all_stocks();

    println!("Averaging changes...");
    let changes = get_all_average_changes(&stocks, step_size);

    println!("Generating bounds...");
    let bounds = get_all_bounds(&changes);

    let config = Config {
        balance: 1000.0,
        buy_amount: 100.0,
        sell_loss_percent: 5.0,
        step_size: step_size,
        start_date: parse_date("2018-01-01".to_string()),
        end_date: parse_date("2020-01-01".to_string()),
    };

    println!("Performing simulation...");
    let result = perform_simulation(stocks, bounds, &config);

    let total_returns = get_change(result.balance, config.balance);
    let duration = config.end_date - config.start_date;
    let years = duration.num_days() as f32 / 365.0;
    let annual_returns = total_returns / years;

    println!("\nFinal balance: ${}", result.balance);
    println!("Total returns: {}%", total_returns);
    println!("Annual returns: {}%", annual_returns);
    println!("{} trades made", result.trades.len());

    save_trades(&result.trades);
}
