fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n < 2 {
            return 1;
        }
        let mut a = 0;
        let mut b = n;
        for i in 2..n {
            let c = a + b;
            a = b;
            b = c;
        }
        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_climb_stairs() {
        [(1, 1), (2, 2), (3, 3), (4, 8)]
            .iter()
            .for_each(|(input, answer)| {
                assert_eq!(Solution::climb_stairs(*input), *answer);
            });
    }
}
