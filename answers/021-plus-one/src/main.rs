#![allow(unused)]
fn main() {
    println!("Hello, world!");
}

struct Solution;
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;

        // For every digit, go in reverse order
        for i in (0..digits.len()).rev() {
            // if the least remaining digit at i is less than 9
            if digits[i] < 9 {
                digits[i] += 1;
                return digits;
            }
            // Digit is 9, then it becomes zero and the carry is 1
            digits[i] = 0;
        }

        let mut result = vec![1];
        result.append(&mut digits);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plus_one() {
        for (input, output) in [
            (vec![1, 2, 3], vec![1, 2, 4]),
            (vec![4, 3, 2, 1], vec![4, 3, 2, 2]),
            (vec![9, 9, 9], vec![1, 0, 0, 0]),
        ] {
            assert_eq!(Solution::plus_one(input), output);
        }
    }
}
