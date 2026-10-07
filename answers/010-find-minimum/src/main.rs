#![allow(unused)]
fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        // iterative
        nums.iter().fold(
            i32::MAX,
            |curr_min, num| if *num < curr_min { *num } else { curr_min },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_min() {
        assert_eq!(Solution::find_min([1, 5, 3, 9, 2].to_vec()), 1);
        assert_eq!(Solution::find_min([-1, -5, -3].to_vec()), -5);
        assert_eq!(Solution::find_min([42].to_vec()), 42);
    }
}
