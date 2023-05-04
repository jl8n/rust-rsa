#[cfg(test)]
mod tests {
    use super::is_prime;
    use crate::primes::{
        generate_prime::{alphabet_to_base10, generate_prime, nearby_prime},
        is_prime::{get_seed, monte_carlo},
    };
    use crate::rsa::extended_euclid;

    use num_bigint::{BigInt, BigUint};

    #[test]
    fn test_generate_prime() {
        let result = generate_prime(&"ab".to_string(), 2);
        assert_eq!(result, BigUint::from(13u8));
        let result = generate_prime(&"yz".to_string(), 4);
        assert_eq!(result, BigUint::from(2531u32));
    }

    #[test]
    fn test_alphabet_to_base10() {
        assert!(alphabet_to_base10(&"abc".to_string()) == BigUint::from(123u8));
        assert!(alphabet_to_base10(&"xyz".to_string()) == BigUint::from(242526u32));
        assert!(alphabet_to_base10(&"~,./;'[]123a".to_string()) == BigUint::from(1u8));
    }

    #[test]
    fn test_nearby_prime() {
        assert!(nearby_prime(BigUint::from(14u8), 2) == BigUint::from(17u8));
        assert!(nearby_prime(BigUint::from(120u8), 3) == BigUint::from(127u8));
        assert!(nearby_prime(BigUint::from(121u8), 3) == BigUint::from(127u8));
        assert!(nearby_prime(BigUint::from(1001u16), 4) == BigUint::from(1009u16));
    }

    #[test]
    fn test_get_seed() {
        let n = BigUint::from(10u8);
        let seed = get_seed(&n);
        assert!(seed >= BigUint::from(2u8));
        assert!(seed < n);
    }

    #[test]
    fn test_monte_carlo() {
        let trial_count = 10;
        let prime_str = b"48950413500958247776918939895110243197568110749207986981599466255471171998956583861825130114208146376913862645586256012246129830589390593869305615234266606932439325212639436201253190830710923034595811";
        let suspect_prime = BigUint::parse_bytes(prime_str, 10).unwrap();
        assert_eq!(monte_carlo(&suspect_prime, &trial_count), true);
        assert_eq!(monte_carlo(&BigUint::from(13u8), &trial_count), true);
        assert_eq!(monte_carlo(&BigUint::from(21u8), &trial_count), false);
    }

    #[test]
    /// Compare miller_rabin accuracy against a deterministic prime test
    fn test_miller_rabin() {
        let mut dif_count = 0u8;
        let upper_bound = 10000u16;
        let trial_count = 10u16;

        for i in 3..upper_bound {
            if monte_carlo(&BigUint::from(i), &trial_count) != is_prime(&BigUint::from(i)) {
                dif_count += 1;
            }
        }

        assert!(dif_count == 0);
    }

    #[test]
    fn test_extended_euclid() {
        let (gcd, x, y) = extended_euclid(&BigInt::from(180u8), &BigInt::from(150u8));
        assert_eq!(gcd, BigInt::from(30u8));
        assert_eq!(x, BigInt::from(1u8));
        assert_eq!(y, BigInt::from(-1i8));
    }
}

use num_bigint::BigUint;
use num_traits::{One, Zero};

/// Deterministic prime test
/// used to test the accuracy of Miller-Rabin
pub fn is_prime(n: &BigUint) -> bool {
    // 1 is not prime
    if n == &BigUint::one() {
        return false;
    }

    let mut i: BigUint = 2u64.into();

    // loop from 2 to int(√x)
    while &i * &i <= *n {
        if n % &i == BigUint::zero() {
            // factor exists between 2 and √x, not prime
            return false;
        }
        i += BigUint::one();
    }

    true
}
