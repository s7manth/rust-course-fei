//! Run this file with `cargo test --test 04_luhn_algorithm`.

// DONE: Implement the Luhn algorithm (https://en.wikipedia.org/wiki/Luhn_algorithm),
// which is used to check the validity of e.g. bank or credit card numbers.

fn luhn_algorithm(number: usize) -> bool {
    if number == 0 {
        return true;
    }

    let mask = number % 10;
    let payload = number / 10;

    let sum: u32 = payload.to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .rev()
        .enumerate()
        .map(|t| { let temp = t.1 * 2; match t.0 % 2 {
            0 => match temp > 9 {
                true => temp - 9,
                false => temp
            }
            1 => t.1,
            _ => 0
        }})
        .sum();

    (10 - sum % 10) == mask as u32
}

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use super::luhn_algorithm;

    #[test]
    fn luhn_zero() {
        assert!(luhn_algorithm(0));
    }

    #[test]
    fn luhn_small_correct() {
        assert!(luhn_algorithm(18));
    }

    #[test]
    fn luhn_small_incorrect() {
        assert!(!luhn_algorithm(10));
    }

    #[test]
    fn luhn_correct() {
        assert!(luhn_algorithm(17893729974));
        assert!(luhn_algorithm(79927398713));
    }

    #[test]
    fn luhn_incorrect() {
        assert!(!luhn_algorithm(17893729975));
        assert!(!luhn_algorithm(17893729976));
        assert!(!luhn_algorithm(17893729977));
        assert!(!luhn_algorithm(123456));
    }
}
