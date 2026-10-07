fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        // naive recursion
        // match n {
        //     0 | 1 => n,
        //     _ => Self::fib(n - 1) + Self::fib(n - 2),
        // }
        // memoization
        // if n < 2 {
        //     return n;
        // }
        // let mut memo = vec![0; n as usize + 1];
        // memo[0] = 0;
        // memo[1] = 1;
        // for i in 2..=n as usize {
        //     memo[i] = memo[i - 1] + memo[i - 2];
        // }
        // memo[n as usize]
        // dynamic programming
        if n < 2 {
            return n;
        }
        let mut a = 0;
        let mut b = 1;
        for _ in 2..=n {
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
    fn test_fib() {
        assert_eq!(Solution::fib(0), 0);
        assert_eq!(Solution::fib(1), 1);
        assert_eq!(Solution::fib(4), 3);
    }
}
