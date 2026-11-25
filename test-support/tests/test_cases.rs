use test_support::test_cases;

struct Solution;

impl Solution {
    fn double(value: i32) -> i32 {
        value * 2
    }

    fn add(left: i32, right: i32) -> i32 {
        left + right
    }

    fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|&x| x != val);
        nums.len() as i32
    }
}

mod function {
    use super::*;

    test_cases! {
        Solution::double;
        zero: 0 => 0,
        positive: 3 => 6
    }
}

mod closure {
    use super::test_cases;

    test_cases! {
        |(left, right)| left + right;
        tuple_input: (2, 3) => 5,
    }
}

mod multiple_arguments {
    use super::*;

    test_cases! {
        Solution::add;
        two_arguments: 2, 3 => 5,
    }

    test_cases! {
        |(left, right), factor| (left + right) * factor;
        tuple_and_scalar: (2, 3), 4 => 20
    }

    test_cases! {
        Solution::remove_element;
        mutable_vector: &mut vec![3, 2, 2, 3], 3 => 2,
    }
}

mod qualified {
    test_support::test_cases! {
        std::convert::identity;
        owned_input: String::from("rust") => "rust",
    }
}

mod expressions {
    use super::*;

    test_cases! {
        multiple_arguments: Solution::add(2, 3) => 5,
        owned_result: "rust".to_uppercase() => String::from("RUST"),
        setup_block: {
            let mut values = vec![3, 1, 2];
            values.sort();
            values
        } => vec![1, 2, 3],
        mutated_input: {
            let mut nums = vec![3, 2, 2, 3];
            let remaining = Solution::remove_element(&mut nums, 3);
            let retained = nums[..remaining as usize].to_vec();
            (remaining, retained)
        } => (2, vec![2, 2]),
    }
}

test_cases! {}
