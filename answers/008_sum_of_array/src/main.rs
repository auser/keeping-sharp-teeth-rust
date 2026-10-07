fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn sum_of_array(nums: Vec<i32>) -> i32 {
        // Iterative
        // nums.iter().sum()
        // Fold
        nums.iter().fold(0, |acc, x| acc + x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_array() {
        assert_eq!(Solution::sum_of_array(vec![1, 2, 3, 4, 5]), 15);
        assert_eq!(Solution::sum_of_array(vec![-1, 0, 1]), 0);
    }
}
