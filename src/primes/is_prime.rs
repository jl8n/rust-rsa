use num_bigint::{BigUint, RandBigInt};
use num_traits::{One, Zero};
use rand::thread_rng;

/// Get a BigUint between 2 and `n`
pub fn get_seed(n: &BigUint) -> BigUint {
    let mut rng = thread_rng();
    let low = BigUint::from(2u32);
    let rand_num: BigUint = rng.gen_biguint_range(&low, &n);
    rand_num
}

/// Nondeterministically runs the Miller-Rabin primality test several times
/// Return true if every Miller-Rabin test predicts a prime
pub fn monte_carlo(n: &BigUint, trial_count: &u16) -> bool {
    for _ in 0..*trial_count {
        let random_seed = get_seed(n);
        let is_still_prime = miller_rabin(&n, &random_seed);

        if !is_still_prime {
            return false;
        }
    }

    true
}

// Miller-Rabin primality test
// Probabilistically determines if a number `n` is prime using
// a random seed from 2 to n-1
pub fn miller_rabin(n: &BigUint, seed: &BigUint) -> bool {
    let two = BigUint::from(2u8);

    if n == &two {
        return true; // 2 is prime
    }

    // Check if n is even (logical AND)
    if n & BigUint::one() == BigUint::zero() {
        return false; // n is composite
    }

    let mut k = BigUint::zero();
    let mut m = n.clone();
    m -= BigUint::from(1u8); // m = n - 1

    // Compute k and m such that n-1 = 2^k * m
    while m.clone() & BigUint::one() == BigUint::zero() {
        // While m is even, increment k and divide m by 2
        k += BigUint::one();
        m >>= 1; // m = m / 2
    }

    // Compute x = seed^m mod n
    let x = seed.modpow(&m, &n);

    // If x is equal to 1 or n-1, then n is probably prime
    if x == BigUint::from(1u8) || x == n - BigUint::from(1u8) {
        return true;
    }

    let mut i = BigUint::from(0u8);

    // Check if n is composite by iterating up to k
    // BigUInt can't be used as an iterator so a while loop is used
    while i < k {
        // Compute x = seed^m mod n
        let x = seed.modpow(&m, &n);

        // If x is equal to n-1 then n is probably prime
        if x == n - BigUint::one() {
            return true;
        } else {
            // Multiply m by two (left bitshift)
            m <<= 1;
        }

        i += 1u8;
    }

    false
}
