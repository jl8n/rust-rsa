#[cfg(test)]
mod tests {
    use super::is_prime;
    use crate::primes::{
        generate_prime::{alphabet_to_base10, generate_prime, nearby_prime},
        is_prime::{get_seed, monte_carlo},
    };
    use crate::rsa::{extended_euclid, generate_rsa_key_components, modular_inv};

    use num_bigint::BigInt;

    #[test]
    fn test_generate_prime() {
        let result = generate_prime(&"ab".to_string(), 2);
        assert_eq!(result, BigInt::from(13u8));
        let result = generate_prime(&"yz".to_string(), 4);
        assert_eq!(result, BigInt::from(2531u32));
    }

    #[test]
    fn test_alphabet_to_base10() {
        assert!(alphabet_to_base10(&"abc".to_string()) == BigInt::from(123u8));
        assert!(alphabet_to_base10(&"xyz".to_string()) == BigInt::from(242526u32));
        assert!(alphabet_to_base10(&"~,./;'[]123a".to_string()) == BigInt::from(1u8));
    }

    #[test]
    fn test_nearby_prime() {
        assert!(nearby_prime(BigInt::from(14u8), 2) == BigInt::from(17u8));
        assert!(nearby_prime(BigInt::from(120u8), 3) == BigInt::from(127u8));
        assert!(nearby_prime(BigInt::from(121u8), 3) == BigInt::from(127u8));
        assert!(nearby_prime(BigInt::from(1001u16), 4) == BigInt::from(1009u16));
    }

    #[test]
    fn test_get_seed() {
        let n = BigInt::from(10u8);
        let seed = get_seed(&n);
        assert!(seed >= BigInt::from(2u8));
        assert!(seed < n);
    }

    #[test]
    fn test_monte_carlo() {
        let trial_count = 10;
        let prime_str = b"48950413500958247776918939895110243197568110749207986981599466255471171998956583861825130114208146376913862645586256012246129830589390593869305615234266606932439325212639436201253190830710923034595811";
        let suspect_prime = BigInt::parse_bytes(prime_str, 10).unwrap();
        assert_eq!(monte_carlo(&suspect_prime, &trial_count), true);
        assert_eq!(monte_carlo(&BigInt::from(13u8), &trial_count), true);
        assert_eq!(monte_carlo(&BigInt::from(21u8), &trial_count), false);
    }

    #[test]
    /// Compare miller_rabin accuracy against a deterministic prime test
    fn test_miller_rabin() {
        let mut dif_count = 0u8;
        let upper_bound = 10000u16;
        let trial_count = 10u16;

        for i in 3..upper_bound {
            if monte_carlo(&BigInt::from(i), &trial_count) != is_prime(&BigInt::from(i)) {
                dif_count += 1;
            }
        }

        assert!(dif_count == 0);
    }

    #[test]
    fn test_extended_euclid() {
        let a = BigInt::from(240);
        let b = BigInt::from(46);
        let (gcd, x, y) = extended_euclid(&a, &b);

        // Verify that the gcd is correct
        let expected_gcd = BigInt::from(2);
        assert_eq!(gcd, expected_gcd);

        // Verify that the bezout coefficients are correct
        let expected_x = BigInt::from(-9);
        let expected_y = BigInt::from(47);
        assert_eq!(x, expected_x);
        assert_eq!(y, expected_y);
    }

    #[test]
    fn test_modular_inv() {
        let a = BigInt::from(5);
        let m = BigInt::from(11);
        let inv = modular_inv(&a, &m).unwrap();

        // The modular inverse of 5 (mod 11) is 9, since 5*9 = 1 (mod 11)
        assert_eq!(inv, BigInt::from(9));

        let a = BigInt::from(4);
        let m = BigInt::from(10);
        let inv = modular_inv(&a, &m);

        // The modular inverse of 4 (mod 10) does not exist, since gcd(4, 10) != 1
        assert!(inv.is_err());
    }

    #[test]
    fn test_generate_rsa_key_components() {
        // Test case 1: Small prime numbers
        let p = BigInt::from(17u32);
        let q = BigInt::from(19u32);
        let (n, e, d) = generate_rsa_key_components(&p, &q);
        assert_eq!(n, BigInt::from(323u32));
        assert_eq!(e, BigInt::from(65537u32));
        assert_eq!(d, BigInt::from(161u32));

        // Test case 2: Large prime numbers
        let p = BigInt::parse_bytes(b"36901412150694956828426050716714926", 10).unwrap();
        let q = BigInt::parse_bytes(b"32384934706043254187408503230356433", 10).unwrap();
        let (n, e, d) = generate_rsa_key_components(&p, &q);
        assert_eq!(
            n,
            BigInt::parse_bytes(
                b"1195049823061047350008871075121144066285225280516043406795321831218958",
                10
            )
            .unwrap()
        );
        assert_eq!(e, BigInt::from(65537u32));
        assert_eq!(
            d,
            BigInt::parse_bytes(
                b"6290983550605937649771282190469424584045852819069413840753542579473",
                10
            )
            .unwrap()
        );

        // Test case 3: Large prime numbers with a zero d value
        let p = BigInt::parse_bytes(b"217744268732466017238455975905810073035", 10).unwrap();
        let q = BigInt::parse_bytes(b"170141183460469231731687303715884105727", 10).unwrap();
        let (n, e, d) = generate_rsa_key_components(&p, &q);
        assert_eq!(
            n,
            BigInt::parse_bytes(
                b"37047267573876214817465981413903539043272556624404862879802077523169531771445",
                10
            )
            .unwrap()
        );
        assert_eq!(e, BigInt::from(65537u32));
        assert_eq!(
            d,
            BigInt::parse_bytes(
                b"33364417454954025221729361353302938513956071561504072396157322167976570035177",
                10
            )
            .unwrap()
        );
    }
}

use num_bigint::BigInt;
use num_traits::{One, Zero};

/// Deterministic prime test
/// used to test the accuracy of Miller-Rabin
pub fn is_prime(n: &BigInt) -> bool {
    // 1 is not prime
    if n == &BigInt::one() {
        return false;
    }

    let mut i: BigInt = 2u64.into();

    // loop from 2 to int(√x)
    while &i * &i <= *n {
        if n % &i == BigInt::zero() {
            // factor exists between 2 and √x, not prime
            return false;
        }
        i += BigInt::one();
    }

    true
}
