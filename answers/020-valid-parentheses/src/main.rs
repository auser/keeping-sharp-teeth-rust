#![allow(unused)]
fn main() {
    println!("Hello, world!");
}

struct Solution;

const OPENING: [char; 3] = ['(', '[', '{'];
const CLOSING: [char; 3] = [')', ']', '}'];

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];
        for ch in s.chars() {
            match ch {
                '(' | '[' | '{' => stack.push(ch),
                ')' => {
                    if stack.pop() != Some('(') {
                        return false;
                    }
                }
                ']' => {
                    if stack.pop() != Some('[') {
                        return false;
                    }
                }
                '}' => {
                    if stack.pop() != Some('{') {
                        return false;
                    }
                }
                _ => {}
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        for (input, b) in [
            ("()", true),
            ("()[]{}", true),
            ("(]", false),
            ("([])", true),
            ("([)]", false),
        ] {
            assert_eq!(Solution::is_valid(input.to_string()), b);
        }
    }
}
