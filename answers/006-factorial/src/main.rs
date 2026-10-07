fn main() {
    println!("{}", Solution::factorial(5));
}

struct Solution;

impl Solution {
    pub fn factorial(n: i32) -> i32 {
        // recursive
        // match n {
        //     0 => 1,
        //     1 => 1,
        //     _ => n * Self::factorial(n - 1),
        // }
        // dynamic programming
        let mut memo = vec![1; (n + 1) as usize];
        for i in 2..=n as usize {
            memo[i] = memo[i - 1] * i as i32;
        }
        memo[n as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(Solution::factorial(0), 1);
        assert_eq!(Solution::factorial(1), 1);
        assert_eq!(Solution::factorial(2), 2);
        assert_eq!(Solution::factorial(3), 6);
        assert_eq!(Solution::factorial(4), 24);
        assert_eq!(Solution::factorial(5), 120);
    }
}
