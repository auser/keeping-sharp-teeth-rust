fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut num = x;
        let mut rev = 0;
        while num > 0 {
            rev = rev * 10 + num % 10;
            num /= 10;
        }
        rev == x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(-121), false);
        assert_eq!(Solution::is_palindrome(10), false);
    }

    #[test]
    fn test_is_palindrome_2() {
        assert_eq!(Solution::is_palindrome(121), true);
    }
}
