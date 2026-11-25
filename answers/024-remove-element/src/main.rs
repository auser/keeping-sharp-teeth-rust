fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        // nums.retain(|&x| x != val);
        // nums.len() as i32
        // or manually
        let mut count = 0;
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[count] = nums[i];
                count += 1;
            }
        }
        nums.truncate(count); // reduce the vector
        count as i32
    }
}

#[cfg(test)]
mod tests {
    use test_support::test_cases;

    // Import your solution here after adding it above:
    use super::Solution;

    test_cases! {
        Solution::remove_element;
        simple_input: &mut vec![3, 2, 2, 3], 3 => 2,
        with_zeros: &mut vec![0,1,2,2,3,0,4,2], 2 => 5,
        really_big_zeros: &mut vec![000000000,0,1,2], 2 => 3
    }
}
