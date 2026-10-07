#![allow(unused)]
fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn is_prime(n: i32) -> bool {
        // 2 is prime
        if n == 2 {
            return true;
        }
        // even is not prime
        if n < 2 || n % 2 == 0 {
            return false;
        }
        // Check odd divisors
        let limit = (n as f64).sqrt() as i32;
        let mut i = 3;
        // While our iterator, starting with 3 is
        // less than the sqrt of n
        while i <= limit {
            // if n is divisible by i, it's not prime
            if n % i == 0 {
                return false;
            }
            i += 2; // only check odd numbers
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        for (num, boolean) in [(7, true), (4, false), (1, false)] {
            assert_eq!(Solution::is_prime(num), boolean);
        }
    }
}
