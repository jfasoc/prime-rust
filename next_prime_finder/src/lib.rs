use num_bigint::{BigUint, ToBigUint};
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
    if n <= &ToBigUint::to_biguint(&2u64).unwrap() {
        return ToBigUint::to_biguint(&2u64).unwrap();
    }

    let mut current = n.clone();
    // If the number is even, and not 2, start with the next odd number.
    if &current != &ToBigUint::to_biguint(&2u64).unwrap() && &current % 2u64 == ToBigUint::to_biguint(&0u64).unwrap() {
        current += 1u64;
    }

    loop {
        if is_prime(&current) {
            return current;
        }
        // For 2, the next prime is 3.
        if &current == &ToBigUint::to_biguint(&2u64).unwrap() {
            current += 1u64;
        } else {
            current += 2u64;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::ToBigUint;

    #[test]
    fn test_is_prime() {
        assert!(!is_prime(&ToBigUint::to_biguint(&0).unwrap()));
        assert!(!is_prime(&ToBigUint::to_biguint(&1).unwrap()));
        assert!(is_prime(&ToBigUint::to_biguint(&2).unwrap()));
        assert!(is_prime(&ToBigUint::to_biguint(&3).unwrap()));
        assert!(!is_prime(&ToBigUint::to_biguint(&4).unwrap()));
        assert!(is_prime(&ToBigUint::to_biguint(&5).unwrap()));
        assert!(!is_prime(&ToBigUint::to_biguint(&6).unwrap()));
        assert!(is_prime(&ToBigUint::to_biguint(&7).unwrap()));
        assert!(!is_prime(&ToBigUint::to_biguint(&9).unwrap()));
        assert!(is_prime(&ToBigUint::to_biguint(&13).unwrap()));
        assert!(is_prime(&"104743".parse().unwrap())); // A known prime
    }

    #[test]
    fn test_next_prime() {
        assert_eq!(next_prime(&ToBigUint::to_biguint(&0).unwrap()), ToBigUint::to_biguint(&2).unwrap());
        assert_eq!(next_prime(&ToBigUint::to_biguint(&1).unwrap()), ToBigUint::to_biguint(&2).unwrap());
        assert_eq!(next_prime(&ToBigUint::to_biguint(&2).unwrap()), ToBigUint::to_biguint(&2).unwrap());
        assert_eq!(next_prime(&ToBigUint::to_biguint(&3).unwrap()), ToBigUint::to_biguint(&3).unwrap());
        assert_eq!(next_prime(&ToBigUint::to_biguint(&4).unwrap()), ToBigUint::to_biguint(&5).unwrap());
        assert_eq!(next_prime(&ToBigUint::to_biguint(&10).unwrap()), ToBigUint::to_biguint(&11).unwrap());
        assert_eq!(next_prime(&ToBigUint::to_biguint(&13).unwrap()), ToBigUint::to_biguint(&13).unwrap());
        assert_eq!(next_prime(&"104740".parse().unwrap()), "104743".parse().unwrap());
    }
}
