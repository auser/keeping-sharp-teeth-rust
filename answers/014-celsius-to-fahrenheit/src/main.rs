#![allow(unused)]
fn main() {
    println!("Hello world")
}

struct Solution;

impl Solution {
    pub fn celsius_to_fahrenheit(celsius: f64) -> f64 {
        (Self::celsius_to_fahrenheit_(celsius) * 100.0) / 100.0
    }

    fn celsius_to_fahrenheit_(celsius: f64) -> f64 {
        (celsius * 9.0 / 5.0) + 32.0
    }

    fn fahrenheit_to_celsius_(farenheit: f64) -> f64 {
        (farenheit - 32.0) * (5.0 / 9.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_celsius_to_fahrenheit() {
        for (input, output) in [(0.0, 32.00), (100.0, 212.00), (37.0, 98.60)] {
            assert_eq!(Solution::celsius_to_fahrenheit(input), output);
        }
    }
}
