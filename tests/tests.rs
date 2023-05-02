use num_bigint::BigUint;
use num_traits::{One, Zero};
use rust_rsa::{monte_carlo, miller_rabin};

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

pub fn test_m_r(upper_bound: &u32, trial_count: &u16) -> u32 {
    let mut dif_count = 0;

    for i in 3..*upper_bound {
        if monte_carlo(&BigUint::from(i), trial_count) != is_prime(&BigUint::from(i)) {
            dif_count += 1;
        }

    }

    dif_count
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monte_carlo() {
        assert_eq!(monte_carlo(&BigUint::from(2u32), &10), true);
        assert_eq!(monte_carlo(&BigUint::from(3u32), &10), true);
        assert_eq!(monte_carlo(&BigUint::from(4u32), &10), false);
        assert_eq!(monte_carlo(&BigUint::from(17u32), &10), true);
        assert_eq!(monte_carlo(&BigUint::from(100u32), &10), false);
    }

    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(&BigUint::from(2u32)), true);
        assert_eq!(is_prime(&BigUint::from(3u32)), true);
        assert_eq!(is_prime(&BigUint::from(4u32)), false);
        assert_eq!(is_prime(&BigUint::from(17u32)), true);
        assert_eq!(is_prime(&BigUint::from(100u32)), false);
    }

    #[test]
    fn test_miller_rabin() {
        let upper_bound = 10000;
        let trial_count = 30;
        let dif_count = test_m_r(&upper_bound, &trial_count);
        println!("Out of {} integers, there were {} differences between Miller-Rabin and a deterministic algorithm.", upper_bound, dif_count);

        assert_eq!(miller_rabin(&BigUint::from(2u32), &BigUint::from(1u32)), true);
        assert_eq!(miller_rabin(&BigUint::from(3u32), &BigUint::from(1u32)), true);
        assert_eq!(miller_rabin(&BigUint::from(4u32), &BigUint::from(1u32)), false);
        assert_eq!(miller_rabin(&BigUint::from(17u32), &BigUint::from(1u32)), true);
        assert_eq!(miller_rabin(&BigUint::from(100u32), &BigUint::from(1u32)), false);
    }
}
