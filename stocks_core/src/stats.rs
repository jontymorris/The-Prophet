use std::collections::HashMap;
use std::iter::Sum;
use super::types::*;

pub fn get_all_bounds(changes: &HashMap<String, Vec<f32>>) -> HashMap<String, Bound> {
    let mut bounds = HashMap::with_capacity(changes.len());

    // calculate bounds for each stock
    for (stock, history) in changes {
        let result = get_bounds(&history);
        bounds.insert(stock.clone(), result);
    }

    return bounds;
}

pub fn get_all_average_changes(stocks: &Vec<Stock>, step_size: usize) -> HashMap<String, Vec<f32>> {
    let mut changes = HashMap::with_capacity(stocks.len());

    // calculate changes for each stock
    for stock in stocks {
        let values = get_average_changes(&stock, step_size);
        changes.insert(stock.symbol.clone(), values);
    }

    return changes;
}

pub fn get_average_changes(stock: &Stock, step_by: usize) -> Vec<f32> {
    let mut average_changes = vec![];
    let mut step_changes = vec![];

    for i in 1..stock.history.len() {
        // calculate today's change
        let today = stock.history[i].close;
        let yesterday = stock.history[i-1].close;
        let change = get_change(today, yesterday);
        
        step_changes.push(change);

        // if at end of period, calculate the average
        if step_changes.len() >= step_by || i == step_changes.len() - 1 {
            let period_mean = get_mean(&step_changes);
            
            average_changes.push(period_mean);
            step_changes.clear();
        }
    }

    return average_changes;
}

pub fn get_bounds(values: &Vec<f32>) -> Bound {
    let mean = get_mean(values);

    // work out standard deviation
    let mut totals: Vec<f32> = Vec::with_capacity(values.len());
    for value in values {
        let x = (value - mean).powf(2.0);
        totals.push(x);
    }

    let standard_deviation = get_mean(&totals).sqrt();

    // consturct the interval
    let interval = 0.608 * standard_deviation;

    return Bound {
        upper: mean + interval,
        lower: mean - interval,
        middle: mean
    };
}

pub fn get_change(value1: f32, value2: f32) -> f32 {
    (value1 - value2) / value2 * 100.0
}

pub fn get_sum(values: &Vec<f32>) -> f32 {
    Sum::sum(values.iter())
}

pub fn get_mean(values: &Vec<f32>) -> f32 {
    get_sum(values) / (values.len() as f32)
}