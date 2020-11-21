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
