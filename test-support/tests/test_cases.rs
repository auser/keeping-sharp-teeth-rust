use test_support::test_cases;

struct Solution;

impl Solution {
    fn double(value: i32) -> i32 {
        value * 2
    }

    fn add(left: i32, right: i32) -> i32 {
        left + right
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
    }
}

test_cases! {}
