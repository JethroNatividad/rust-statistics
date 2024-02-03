// Program that takes in multiple numbers, and shows the average, min, max, and standard deviation.
// Inputs: numbers
// Process: calculate average, min, max, and standard deviation.
// Outputs: show average, min, max, and standard deviation.

fn calculate_average(vector: Vec<f64>) -> f64 {
    // set sum
    // loop vector
    // add to sum

    // divide sum / vector
    // return
}

fn calculate_min(vector: Vec<f64>) -> f64 {
    // set min = f64max
    // loop vector
    // min item and min

    // return min
}

fn calculate_max(vector: Vec<f64>) -> f64 {
    // set max = f64min
    // loop vector
    // max item and max

    // return max
}

fn calculate_standard_deviation(vector: Vec<f64>) -> f64 {
    // get average
    // set variance = 0
    // loop vector
    // variance += (item - average)**2

    // standard deviation = sqrt variance
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_average() {
        // calculate_average
        assert_eq!(calculate_average(vec![100.0, 200.0, 1000.0, 300.0]), 400.0);

        assert_eq!(calculate_average(vec![52.0, 60.0, 75.0, 80.0, 90.0]), 71.4);
        assert_eq!(
            calculate_average(vec![100.0, 150.0, 200.0, 250.0, 300.0]),
            200.0
        );
        assert_eq!(calculate_average(vec![55.0, 65.0, 75.0, 85.0, 95.0]), 75.0);
    }

    #[test]
    fn test_calculate_min() {
        // calculate min
        assert_eq!(calculate_average(vec![100.0, 200.0, 1000.0, 300.0]), 100.0);

        assert_eq!(calculate_min(vec![52.0, 60.0, 75.0, 80.0, 90.0]), 52.0);
        assert_eq!(
            calculate_min(vec![100.0, 150.0, 200.0, 250.0, 300.0]),
            100.0
        );
        assert_eq!(calculate_min(vec![55.0, 65.0, 75.0, 85.0, 95.0]), 55.0);
    }

    #[test]
    fn test_calculate_max() {
        // calculate max
        assert_eq!(calculate_average(vec![100.0, 200.0, 1000.0, 300.0]), 1000.0);

        assert_eq!(calculate_max(vec![52.0, 60.0, 75.0, 80.0, 90.0]), 90.0);
        assert_eq!(
            calculate_max(vec![100.0, 150.0, 200.0, 250.0, 300.0]),
            300.0
        );
        assert_eq!(calculate_max(vec![55.0, 65.0, 75.0, 85.0, 95.0]), 95.0);
    }

    #[test]
    fn test_calculate_standard_deviation() {
        // calculate standard deviation
        assert_eq!(calculate_average(vec![100.0, 200.0, 1000.0, 300.0]), 400.25);

        assert_eq!(
            calculate_standard_deviation(vec![52.0, 60.0, 75.0, 80.0, 90.0]),
            13.37
        );
        assert_eq!(
            calculate_standard_deviation(vec![100.0, 150.0, 200.0, 250.0, 300.0]),
            70.71
        );
        assert_eq!(
            calculate_standard_deviation(vec![55.0, 65.0, 75.0, 85.0, 95.0]),
            14.14
        );
    }
}
fn main() {
    println!("Hello, world!");
}
