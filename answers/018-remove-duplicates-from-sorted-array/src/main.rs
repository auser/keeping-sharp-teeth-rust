#![allow(unused)]
fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        // Position for the next element
        let mut curr = 1;

        // For every element int he array
        for next in 1..nums.len() {
            // If the next elem is not the same as the unique last element
            // e.g. write over the last one
            if nums[next] != nums[curr - 1] {
                nums[curr] = nums[next];
                curr += 1;
            }
        }

        curr as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        for (mut arr, ans) in [(vec![1, 1, 2], 2), (vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4], 5)] {
            assert_eq!(Solution::remove_duplicates(&mut arr), ans);
        }
    }
}
