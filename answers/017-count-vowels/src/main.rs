#![allow(unused)]
fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn count_vowels(s: String) -> i32 {
        // iterative version
        let vowels = ['a', 'e', 'i', 'o', 'u'];
        // let mut count = 0;
        // for ch in s.chars() {
        //     if vowels.contains(&ch.to_ascii_lowercase()) {
        //         count += 1;
        //     }
        // }
        // count
        // rusty version
        s.chars()
            .filter(|c| vowels.contains(&c.to_ascii_lowercase()))
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_vowels() {
        for (input, output) in [("Hello World", 3), ("AEIOU", 5), ("xyz", 0)] {
            assert_eq!(Solution::count_vowels(input.to_string()), output);
        }
    }
}
