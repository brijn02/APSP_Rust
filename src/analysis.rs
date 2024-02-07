// The functions to return standard statistical measures for the analysis of the data
// 30-01-2024

// Function to compute the average. If 0 values then return 0.
pub fn compute_average(values: &Vec<f64>) -> f64 {
    let sum: f64 = values.iter().cloned().sum();
    let count = values.len();

    if count == 0 {
        0.0 // Avoid division by zero
    } else {
        sum / count as f64
    }
}

// Function to compute the standard deviation
pub fn compute_std(values: &Vec<f64>, average: f64) -> f64 {
    let mut sum_errors_squared = 0.0;
    for &value in values {
        sum_errors_squared += (value - average).powi(2);
    }
    let mean_squared_error = sum_errors_squared / (values.len() - 1) as f64;
    mean_squared_error.sqrt()
}