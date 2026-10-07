fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn count_digits(num: i32) -> i32 {
        // Iterative
        // let mut count = 0;
        // let mut n = num;
        // while n > 0 {
        //     count += 1;
        //     n /= 10;
        // }
        // count
        // Logarithmic
        let count = (num as f64).log10().floor() as i32 + 1;
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_digits() {
        assert_eq!(Solution::count_digits(123), 3);
    }

    #[test]
    fn test_count_digits_2() {
        assert_eq!(Solution::count_digits(1234), 4);
    }

    #[test]
    fn test_count_digits_3() {
        assert_eq!(Solution::count_digits(1000000), 7);
    }
}
