use num_bigint::BigUint;
use is_prime as is_prime_ext;

/// Checks if a number is prime.
///
/// This function uses the `is_prime` crate, which uses the Miller-Rabin
/// primality test for large numbers.
///
/// # Arguments
///
/// * `n` - A `BigUint` to check for primality.
///
/// # Returns
///
/// * `true` if `n` is prime, `false` otherwise.
pub fn is_prime(n: &BigUint) -> bool {
    is_prime_ext::is_prime(&n.to_string())
}

/// Finds the next prime number greater than or equal to `n`.
///
/// This function starts from `n` and iteratively checks the next numbers
/// until a prime is found.
///
/// # Arguments
///
/// * `n` - A `BigUint` to start searching from.
///
/// # Returns
///
/// * The next prime number as a `BigUint`.
pub fn next_prime(n: &BigUint) -> BigUint {
    if n <= &BigUint::from(2u64) {
        return BigUint::from(2u64);
    }

    let mut current = n.clone();
    // If the number is even, start with the next odd number.
    if &current % 2u64 == BigUint::from(0u64) {
        current += 1u64;
    }

    loop {
        if is_prime(&current) {
            return current;
        }
        current += 2u64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        assert!(!is_prime(&BigUint::from(0u64)));
        assert!(!is_prime(&BigUint::from(1u64)));
        assert!(is_prime(&BigUint::from(2u64)));
        assert!(is_prime(&BigUint::from(3u64)));
        assert!(!is_prime(&BigUint::from(4u64)));
        assert!(is_prime(&BigUint::from(5u64)));
        assert!(!is_prime(&BigUint::from(6u64)));
        assert!(is_prime(&BigUint::from(7u64)));
        assert!(!is_prime(&BigUint::from(9u64)));
        assert!(is_prime(&BigUint::from(13u64)));
        assert!(is_prime(&"104743".parse().unwrap())); // A known prime
    }

    #[test]
    fn test_next_prime() {
        assert_eq!(next_prime(&BigUint::from(0u64)), BigUint::from(2u64));
        assert_eq!(next_prime(&BigUint::from(1u64)), BigUint::from(2u64));
        assert_eq!(next_prime(&BigUint::from(2u64)), BigUint::from(2u64));
        assert_eq!(next_prime(&BigUint::from(3u64)), BigUint::from(3u64));
        assert_eq!(next_prime(&BigUint::from(4u64)), BigUint::from(5u64));
        assert_eq!(next_prime(&BigUint::from(10u64)), BigUint::from(11u64));
        assert_eq!(next_prime(&BigUint::from(13u64)), BigUint::from(13u64));
        assert_eq!(
            next_prime(&"104740".parse().unwrap()),
            "104743".parse().unwrap()
        );
    }
}
