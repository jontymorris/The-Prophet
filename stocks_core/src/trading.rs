use super::stats::{get_best_fit,get_percent_change};

pub fn should_sell(closes: &Vec<f64>, buy_price: f64, sell_loss_percent: f64, sell_gain_percent: f64,) -> bool {
    let latest_price = *closes.last().unwrap();
    
    // auto sell if we have reached our margins
    let current_gains = get_percent_change(latest_price, buy_price);
    if current_gains >= sell_gain_percent || current_gains <= -sell_loss_percent {
        return true;
    }
    
    // find the latest peak
    let mut latest_peak_index = 0;
    let mut latest_peak_value = closes.first().unwrap();
    for (index, value) in closes.iter().enumerate() {
        if value > latest_peak_value {
            latest_peak_index = index;
            latest_peak_value = value;
        }
    }

    // now find the lowest value after peak
    let mut recent_low_index = latest_peak_index;
    let mut recent_low_value = latest_peak_value;
    for (index, value) in closes.iter().enumerate() {
        // skip till we're past the peak index
        if index < recent_low_index {
            continue;
        }

        // don't include the last index
        if index == closes.len() - 1 {
            continue;
        }

        if value < recent_low_value {
            recent_low_index = index;
            recent_low_value = value;
        }
    }

    // have we reached the peak's low?
    if latest_price <= *recent_low_value {
        // check we haven't found the peak
        if recent_low_index != latest_peak_index {
            // also don't care about daily decreases
            if closes.len() - recent_low_index > 2 {
                return true;
            }
        }
    }

    return false;
}

pub fn should_buy(closes: &Vec<f64>) -> bool {
    let latest_price = *closes.last().unwrap();

    // ignore penny stocks
    if latest_price < 1.0 {
        return false;
    }

    // now see if the trend is going up
    let (gradient, r) = get_best_fit(&closes);
    if gradient > 0.025 && r > 0.85 {
        // todo: do we really want to limit the gradient?
        // often an abnormal spike get's sold away quick...
        // maybe this should be optimized for the target market
        if gradient < 0.035 {
            return true;
        }
    }
    
    return false;
}

pub fn get_risk_factored_amount(max_amount: f64, closes: &Vec<f64>) -> f64 {
    // score the volatility, and reduce max amount if above threashhold
    // if volatility too high, then don't buy
    
    return max_amount;
}
