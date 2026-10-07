#![allow(unused)]
fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn swap(a: i32, b: i32) -> (i32, i32) {
        // Self::inplace_math(a, b)
        Self::xor_swap(a, b)
    }

    fn implicit_swap(a: i32, b: i32) -> (i32, i32) {
        (b, a)
    }

    fn inplace_math(a: i32, b: i32) -> (i32, i32) {
        let a = a + b;
        let b = a - b;
        let a = (a - b);
        (a, b)
    }

    fn xor_swap(a: i32, b: i32) -> (i32, i32) {
        let a = a ^ b;
        let b = a ^ b;
        let a = a ^ b;
        (a, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap() {
        [(5, 10), (-1, 1), (0, 0)].iter().for_each(|(a, b)| {
            assert_eq!(Solution::swap(*a, *b), (*b, *a));
        })
    }
}
