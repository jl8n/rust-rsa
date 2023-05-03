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

pub fn test_m_r(upper_bound: &u32, trial_count: &u16) -> u32 {
    let mut dif_count = 0;

    for i in 3..*upper_bound {
        if monte_carlo(&BigUint::from(i), trial_count) != is_prime(&BigUint::from(i)) {
            dif_count += 1;
        }
    }

    dif_count
}
