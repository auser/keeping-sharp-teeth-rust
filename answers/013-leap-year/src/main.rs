#![allow(unused)]

fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn is_leap_year(year: i32) -> bool {
        // Divisible by 4
        //  AND
        //  NOT divisible by 100
        //  OR  divisible by 400
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_leap_year() {
        for (year, res) in [(2000, true), (1900, false), (2024, true)] {
            assert_eq!(Solution::is_leap_year(year), res);
        }
    }
}
