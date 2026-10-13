#![allow(unused)]
fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x < 2 {
            return x;
        }

        let x = x as i64;
        let mut left: i64 = 1;
        let mut right: i64 = x / 2;

        while left <= right {
            let mid = left + (right - left) / 2;
            let square = mid * mid;

            if square == x {
                return mid as i32;
            } else if square < x {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        right as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_sqrt() {
        for (x, res) in [(4, 2), (8, 2)] {
            assert_eq!(Solution::my_sqrt(x), res);
        }
    }
}
