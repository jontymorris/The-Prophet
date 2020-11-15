mod data;
mod simulator;

use data::{get_all_stocks, save_trades};
use simulator::*;
use stocks_core::stats::*;
use stocks_core::util::parse_date;

fn main() {
    let step_size = 3;

    println!("Preparing stocks...");
    let stocks = get_all_stocks("us".into());
    let changes = get_all_average_changes(&stocks, 7);
    let bounds = get_all_bounds(&changes);

    let config = Config {
        balance: 1000.0,
        buy_amount: 100.0,
        sell_loss_percent: 5.0,
        step_size: step_size,
        start_date: parse_date("2015-01-01".to_string()),
        end_date: parse_date("2019-10-01".to_string()),
    };

    println!("Running simulation...");
    let result = perform_simulation(stocks, bounds, &config);

    println!("\nFinal balance: ${}", result.balance);
    println!("{} trades made", result.trades.len());

    let total_returns = get_change(result.balance, config.balance);
    let duration = config.end_date - config.start_date;
    let years = duration.num_days() as f32 / 365.0;
    let annual_returns = total_returns / years;

    println!("\nTotal returns: {}%", total_returns);
    println!("Annual returns: {}%", annual_returns);

    save_trades(&result.trades);
}
