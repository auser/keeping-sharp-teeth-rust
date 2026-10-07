fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn find_max(nums: Vec<i32>) -> i32 {
        let mut max = nums.first();
        for num in nums.iter() {
            if num > max {
                max = num;
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_max_test() {
        assert_eq!(Solution::find_max([1, 5, 3, 9, 2].to_vec()), 9);
        assert_eq!(Solution::find_max([-1, -5, -3].to_vec()), -1);
        assert_eq!(Solution::find_max([42].to_vec()), 42);
    }
}
