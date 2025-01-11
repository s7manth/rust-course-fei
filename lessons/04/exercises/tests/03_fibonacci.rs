//! Run this file with `cargo test --test 03_fibonacci`.

//! DONE: Implement a struct called `Fibonacci`, which implements `Iterator` that iterates through
//! Fibonacci numbers (starting from 0).
//! `Fibonacci` should implement the `Default` trait.

use std::iter::Iterator;

struct Fibonacci {
    a: u64, 
    b: u64
}

impl Fibonacci {
    fn default() -> Self {
        Self {
            a: 0,
            b: 1
        }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.a;
        self.a = self.b;
        self.b += result;
        Some(result)
    }
}

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use crate::Fibonacci;

    #[test]
    fn fibonacci_first() {
        assert_eq!(Fibonacci::default().next(), Some(0u64));
    }

    #[test]
    fn fibonacci_ten() {
        assert_eq!(
            Fibonacci::default().take(10).collect::<Vec<_>>(),
            vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
        );
    }

    #[test]
    fn fibonacci_twenty() {
        assert_eq!(Fibonacci::default().nth(19), Some(4181));
    }

    #[test]
    fn fibonacci_sixty() {
        assert_eq!(Fibonacci::default().nth(59), Some(956722026041));
    }
}
