#![allow(unused)]
fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn merge(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let (mut left, mut right) = (0, 0);
        let mut res = Vec::with_capacity(nums1.len() + nums2.len());

        while left < nums1.len() && right < nums2.len() {
            if nums1[left] <= nums2[right] {
                res.push(nums1[left]);
                left += 1;
            } else {
                res.push(nums2[right]);
                right += 1;
            }
        }

        res.extend_from_slice(&nums1[left..]);
        res.extend_from_slice(&nums2[right..]);

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge() {
        for (input1, input2, answer) in [
            (vec![1, 2, 3], vec![2, 5, 6], vec![1, 2, 2, 3, 5, 6]),
            (vec![1], vec![], vec![1]),
            (vec![], vec![1], vec![1]),
        ] {
            assert_eq!(Solution::merge(input1, input2), answer);
        }
    }
}
