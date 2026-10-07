fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut res = Vec::new();
        for i in 1..=n {
            if i % 3 == 0 && i % 5 == 0 {
                res.push("FizzBuzz".to_string());
            } else if i % 3 == 0 {
                res.push("Fizz".to_string());
            } else if i % 5 == 0 {
                res.push("Buzz".to_string());
            } else {
                res.push(i.to_string());
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fizz_buzz() {
        assert_eq!(Solution::fizz_buzz(3), vec!["1", "2", "Fizz"]);
    }
    #[test]
    fn test_fizz_buzz_2() {
        assert_eq!(Solution::fizz_buzz(5), vec!["1", "2", "Fizz", "4", "Buzz"]);
    }
    #[test]
    fn test_fizz_buzz_3() {
        assert_eq!(
            Solution::fizz_buzz(15),
            vec![
                "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz",
                "13", "14", "FizzBuzz"
            ]
        );
    }
}
