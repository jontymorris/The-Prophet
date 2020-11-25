use super::stats::*;

pub fn should_sell(
    closes: &Vec<f64>,
    buy_price: f64,
    sell_loss_percent: f64,
    sell_gain_percent: f64,
) -> bool {
    let latest_price = *closes.last().unwrap();
    let current_gains = get_percent_change(latest_price, buy_price);

    // sell if we've reached the loss margin
    if current_gains <= -sell_loss_percent {
        return true;
    }

    // hold on if we haven't reach the gain margin
    if current_gains <= sell_gain_percent {
        return false;
    }

    // now wait till our profit maximises
    return is_at_recent_low(&closes[closes.len() - 10..].to_vec());
}

pub fn should_buy(closes: &Vec<f64>) -> bool {
    // ignore penny stocks
    let latest_price = *closes.last().unwrap();
    if latest_price < 1.0 {
        return false;
    }

    // stay away from dying stocks
    let (overall_gradient, _) = get_best_fit(&closes);
    if overall_gradient < 0.00 {
        return false;
    }

    let recent_days = 4;
    let length = closes.len();

    let (old_values, new_values) = closes.split_at(length - recent_days);
    let (old_gradient, old_r) = get_best_fit(&old_values[old_values.len() - 14..].to_vec());
    let (new_gradient, new_r) = get_best_fit(&new_values.to_vec());

    // no recent decreases
    if new_values[recent_days - 1] < new_values[recent_days - 2]
        || new_values[recent_days - 2] < new_values[recent_days - 3]
    {
        return false;
    }

    // recent difference isn't too much
    let recent_difference = get_percent_change(*new_values.last().unwrap(), *old_values.last().unwrap());
    if recent_difference.abs() >= 1.0 {
        return false;
    }

    // the trend correlation is strong enough
    if old_r.abs() < 0.83 || new_r < 0.83 {
        return false;
    }

    // new trend is in right range
    if new_gradient < 0.010 || new_gradient > 0.030 {
        return false;
    }

    // check the gradient difference is in right range
    let gradient_difference = get_percent_change(new_gradient, old_gradient);
    if gradient_difference.abs() < 30.0 && gradient_difference.abs() < 100.0 {
        return false;
    }
    
    return true; // looks good to me!
}

pub fn get_risk_factored_amount(max_amount: f64, closes: &Vec<f64>) -> f64 {
    let average_volatility = get_average_volatility(closes);
    let inverse = 1.0 / average_volatility;

    // invest less in risky stocks
    if inverse < 0.3 {
        return max_amount * 0.5;
    }

    return max_amount;
}
