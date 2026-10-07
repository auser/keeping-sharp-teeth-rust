#![allow(unused)]
fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn even_or_odd(n: i32) -> String {
        // Single modulo
        // match n % 2 {
        //     0 => "Even".to_string(),
        //     _ => "Odd".to_string(),
        // }
        // bitwise
        match n & 1 {
            1 => "Odd".to_string(),
            _ => "Even".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_even_or_odd() {
        for (num, s) in [(4, "Even"), (7, "Odd"), (0, "Even")] {
            assert_eq!(Solution::even_or_odd(num), s)
        }
    }
}
