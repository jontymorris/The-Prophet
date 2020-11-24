use rgsl::fit::linear;
use rgsl::statistics::correlation;

pub fn get_percent_change(new_value: f64, previous_value: f64) -> f64 {
    (new_value - previous_value) / previous_value * 100.0
}

pub fn get_best_fit(values: &Vec<f64>) -> (f64, f64) {
    let mut x_points: Vec<f64> = Vec::with_capacity(values.len());
    for i in 0..values.len() {
        x_points.push(i as f64);
    }

    let (_, _, gradient, _, _, _, _) = linear(&x_points, 1, &values, 1, values.len());
    let r = correlation(&x_points, 1, &values, 1, values.len());

    return (gradient, r);
}

pub fn is_at_recent_low(values: &Vec<f64>) -> bool {
    let last_value = values.last().unwrap();
    
    // find the latest peak
    let mut latest_peak_index = 0;
    let mut latest_peak_value = values.first().unwrap();
    for (index, value) in values.iter().enumerate() {
        if value > latest_peak_value {
            latest_peak_index = index;
            latest_peak_value = value;
        }
    }

    // now find the lowest value after peak
    let mut recent_low_index = latest_peak_index;
    let mut recent_low_value = latest_peak_value;
    for (index, value) in values.iter().enumerate() {
        // skip till we're past the peak index
        if index < recent_low_index {
            continue;
        }

        // don't include the last index
        if index == values.len() - 1 {
            continue;
        }

        if value < recent_low_value {
            recent_low_index = index;
            recent_low_value = value;
        }
    }

    // have we reached the peak's low?
    if last_value <= recent_low_value {
        // check we haven't found the peak
        if recent_low_index != latest_peak_index {
            // also don't care if it was yeseterday
            if values.len() - recent_low_index > 2 {
                return true;
            }
        }
    }

    return false;
}

pub fn get_average_volatility(values: &Vec<f64>) -> f64 {
    let mut weekly_changes = 0.0;
    let mut num_of_periods = 0.0;

    for chunk in values.chunks(7) {
        let mut current_high = 0.0;
        let mut current_low = 0.0;
        
        for value in chunk {
            if *value > current_high {
                current_high = *value;
            }
    
            if *value < current_low {
                current_low = *value;
            }
        }

        weekly_changes += get_percent_change(current_high, current_low);
        num_of_periods += 1.0;
    }

    if num_of_periods == 0.0 {
        return weekly_changes;
    }

    return weekly_changes / num_of_periods;
}
