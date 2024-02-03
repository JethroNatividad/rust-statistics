use std::io;
use std::io::Write;

// Program that takes in multiple numbers, and shows the average, min, max, and standard deviation.
// Inputs: numbers
// Process: calculate average, min, max, and standard deviation.
// Outputs: show average, min, max, and standard deviation.

fn round_decimal(number: f64) -> f64 {
    (number * 100.0).round() / 100.0
}

fn calculate_average(vector: Vec<f64>) -> f64 {
    // set sum
    let mut sum: f64 = 0.0;
    // loop vector
    // add to sum
    for item in &vector {
        sum += item;
    }

    // divide sum / vector
    let average: f64 = sum / vector.len() as f64;
    // return
    average
}

fn calculate_min(vector: Vec<f64>) -> f64 {
    // set min = f64max
    let mut min: f64 = f64::MAX;

    // loop vector
    // min item and min

    for item in &vector {
        min = item.min(min);
    }

    // return min
    min
}

fn calculate_max(vector: Vec<f64>) -> f64 {
    // set max = f64min
    let mut max: f64 = f64::MIN;

    // loop vector
    // max item and max
    for item in &vector {
        max = item.max(max);
    }

    // return max
    max
}

fn calculate_standard_deviation(vector: Vec<f64>) -> f64 {
    // get average
    let average: f64 = calculate_average(vector.clone());
    // set squared_diff = 0
    let mut squared_diff_sum: f64 = 0.0;
    // loop vector
    // squared_diff += (item - average)**2

    for item in &vector {
        squared_diff_sum += (item - average).powf(2.0);
    }

    let mean_of_squared_diff_sum: f64 = squared_diff_sum / vector.len() as f64;

    // standard deviation = sqrt variance
    let standard_deviation: f64 = mean_of_squared_diff_sum.sqrt();
    round_decimal(standard_deviation)
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
        assert_eq!(calculate_min(vec![100.0, 200.0, 1000.0, 300.0]), 100.0);

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
        assert_eq!(calculate_max(vec![100.0, 200.0, 1000.0, 300.0]), 1000.0);

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
        assert_eq!(
            calculate_standard_deviation(vec![100.0, 200.0, 1000.0, 300.0]),
            353.55
        );

        assert_eq!(
            calculate_standard_deviation(vec![52.0, 60.0, 75.0, 80.0, 90.0]),
            13.71
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

fn get_input<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse() {
            Ok(value) => break value,
            Err(_) => println!("Invalid input. Please try again."),
        }
    }
}
fn main() {
    // init empty vec
    let mut numbers: Vec<f64> = vec![];

    // loop
    loop {
        // get input. "Enter a number: "
        let input: String = get_input("Enter a number: ");
        if input.to_lowercase().as_str() == "done" {
            break;
        }

        // Convert to f64
        match input.parse::<f64>() {
            Ok(value) => numbers.push(value),
            Err(_) => println!("Invalid Input, Please try again."),
        }
    }

    // show numbers, "Numbers: ..."
    // get average, minimum, maximum, standard_deviation.

    // print, "The average is {}."
    // print, "The minimum is {}."
    // print, "The maximum is {}."
    // print, "The standard deviation is {}."
}
