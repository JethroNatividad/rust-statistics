// Program that takes in multiple numbers, and shows the average, min, max, and standard deviation.
// Inputs: numbers
// Process: calculate average, min, max, and standard deviation.
// Outputs: show average, min, max, and standard deviation.
// round numbers

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_average() {
        // calculate_average
        assert_eq!(calculate_average(vec![52, 60, 75, 80, 90]), 71);
        assert_eq!(calculate_average(vec![100, 150, 200, 250, 300]), 200);
        assert_eq!(calculate_average(vec![55, 65, 75, 85, 95]), 75);
    }

    #[test]
    fn test_calculate_min() {
        // calculate min
        assert_eq!(calculate_min(vec![52, 60, 75, 80, 90]), 52);
        assert_eq!(calculate_min(vec![100, 150, 200, 250, 300]), 100);
        assert_eq!(calculate_min(vec![55, 65, 75, 85, 95]), 55);
    }

    #[test]
    fn test_calculate_max() {
        // calculate max
        assert_eq!(calculate_max(vec![52, 60, 75, 80, 90]), 90);
        assert_eq!(calculate_max(vec![100, 150, 200, 250, 300]), 300);
        assert_eq!(calculate_max(vec![55, 65, 75, 85, 95]), 95);
    }
}
fn main() {
    println!("Hello, world!");
}
