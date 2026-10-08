#![allow(unused)]

fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        // Create a mutable input
        let mut num = x;
        // the reversed integer
        let mut reversed: i32 = 0;
        // While the number is not equal to 0
        while num != 0 {
            let digit = num % 10; // get the last digit
            // Check boundaries
            // if the reversed integer is larger than the maximum number
            // as a digit
            // and reversed is equal to the max number and
            // the new digit is larger than the maximum addition
            if reversed > i32::MAX / 10 && (reversed == i32::MAX && digit > 7) {
                return 0;
            }
            // if the reversed number is smaller than the minimum decimal
            // value and
            // the reversed integer is equal to the absolute minimum
            // and the digit will shrink the number further, return 0
            if reversed < i32::MIN / 10 && (reversed == i32::MIN && digit < -8) {
                return 0;
            }
            // Add the reversed integer with the last digit
            reversed = reversed * 10 + digit;
            // Continue shrinking the number
            num /= 10
        }
        reversed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        for (input, output) in [(123, 321), (-123, -321), (120, 21)] {
            assert_eq!(Solution::reverse(input), output);
        }
    }
}
