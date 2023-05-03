#[cfg(test)]
mod tests {
    use crate::primes::is_prime::{miller_rabin, monte_carlo};

    use num_bigint::BigUint;
    //use crate::primes::is_prime;

    #[test]
    fn test_monte_carlo() {
        let trial_count = 10;
        let prime_str = b"48950413500958247776918939895110243197568110749207986981599466255471171998956583861825130114208146376913862645586256012246129830589390593869305615234266606932439325212639436201253190830710923034595811";
        let suspect_prime = BigUint::parse_bytes(prime_str, 10).unwrap();
        assert_eq!(monte_carlo(&suspect_prime, &trial_count), true);
    }

    // #[test]
    // fn test_is_prime() {
    //     assert_eq!(is_prime(&BigUint::from(2u32)), true);
    //     assert_eq!(is_prime(&BigUint::from(3u32)), true);
    //     assert_eq!(is_prime(&BigUint::from(4u32)), false);
    //     assert_eq!(is_prime(&BigUint::from(17u32)), true);
    //     assert_eq!(is_prime(&BigUint::from(100u32)), false);
    // }

    #[test]
    fn test_miller_rabin() {
        assert_eq!(
            miller_rabin(&BigUint::from(2u32), &BigUint::from(1u32)),
            true
        );
        assert_eq!(
            miller_rabin(&BigUint::from(3u32), &BigUint::from(1u32)),
            true
        );
        assert_eq!(
            miller_rabin(&BigUint::from(4u32), &BigUint::from(1u32)),
            false
        );
        assert_eq!(
            miller_rabin(&BigUint::from(17u32), &BigUint::from(1u32)),
            true
        );
        assert_eq!(
            miller_rabin(&BigUint::from(100u32), &BigUint::from(1u32)),
            false
        );
    }
}
