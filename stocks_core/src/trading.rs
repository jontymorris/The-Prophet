use super::stats::*;
use super::types::*;

pub fn should_sell(latest_close: &Close, stock_bound: &Bound, bought_price: f32) -> bool {
    let current_margin = get_change(latest_close.value, bought_price);

    // sell if we reach our stop-margin
    if current_margin <= -5.0 {
        return true;
    }

    // sell if we're in a profitable peak
    if latest_close.percent_change >= stock_bound.upper {
        if current_margin > 0.5 {
            return true;
        }
    }

    return false;
}

pub fn should_buy(latest_close: &Close, stock_bound: &Bound) -> bool {
    // ignore penny stocks
    if latest_close.value <= 1.0 {
        return false;
    }

    // ignore down-trending stocks
    if stock_bound.middle <= 0.1 {
        return false;
    }

    // buy if we're in a dip
    if latest_close.percent_change < stock_bound.lower {
        return true;
    }

    return false;
}
