fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let (mut a, mut b) = (1, 1);

        for _ in 2..=n {
            (a, b) = (b, a + b);
        }
        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_support::test_cases;

    test_cases! {
        Solution::climb_stairs;
        single_step: 1 => 1,
        two_steps: 2 => 2,
        three_steps: 3 => 3,
        four_steps: 4 => 5,
        five_steps: 5 => 8,
    }

    #[test]
    fn test_climb_stairs() {
        [(1, 1), (2, 2), (3, 3), (4, 5)]
            .iter()
            .for_each(|(input, answer)| {
                assert_eq!(Solution::climb_stairs(*input), *answer);
            });
    }
}
