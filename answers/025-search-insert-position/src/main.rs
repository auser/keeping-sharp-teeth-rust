fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
  pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    // Manual
    //     let mut left = 0;
    //     let mut right = nums.len();
    //
    //     while left < right {
    //       let mid = left + (right - left) / 2;
    //
    //       if nums[mid] < target {
    //         left = mid + 1;
    //       } else {
    //         right = mid;
    //       }
    //     }
    //     left as i32
    match nums.binary_search(&target) {
      Ok(idx) | Err(idx) => idx as i32
    }
  }
}


#[cfg(test)]
mod tests {
    use super::Solution;
    use test_support::test_cases;

    test_cases! {
      Solution::search_insert;
      target_five: vec![1,3,5,6], 5 => 2,
      target_two: vec![1,3,5,6], 2 => 1,
      target_four: vec![1,3,5,6], 7 => 4,
    }
}
