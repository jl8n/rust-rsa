use rand::thread_rng;
use num_bigint::{BigUint, RandBigInt};
use num_traits::{One, Zero};
use std::time::Instant;


fn get_seed(n: &BigUint) -> BigUint {
    let mut rng = thread_rng();
    let low = BigUint::from(2u32);
    let rand_num: BigUint = rng.gen_biguint_range(&low, &n);
    rand_num
}
 
/// Executes the Miller-Rabin primality test several times
/// Return true if every Miller-Rabin test predicts a prime
fn monte_carlo(n: &BigUint, trial_count: &u16) -> bool {
    for _ in 0..*trial_count {
        let random_seed = get_seed(n);
        let is_still_prime = miller_rabin(&n, &random_seed);

        if !is_still_prime {
            return false;
        }
    }

    true
}


fn miller_rabin(n: &BigUint, seed: &BigUint) -> bool {
    let two = BigUint::from(2u8);

    if n == &two {
        return true;
    }
    if n % &two == BigUint::zero() {
        return false;
    }

    //let mut k = BigInt::from(0);
    let mut k = BigUint::zero();
    let mut m = n.clone();
    m -= BigUint::from(1u8);  // m = n - 1

    while m.clone() & BigUint::one() == BigUint::zero() {
        k += BigUint::one();
        m >>= 1;  // m = m / 2
    }

    if seed.modpow(&m, &n) == BigUint::from(1u8) {
        return true;
    }

    let mut i = BigUint::from(0u8);

    while i < k {
        if seed.modpow(&m, &n) == n - BigUint::one() {
            return true;
        } else {
            m <<= 1;  // m = m * 2
        }

        i += 1u8;
    }

    false
}


/// Deterministic prime test
/// used to test the accuracy of Miller-Rabin
fn is_prime(n: &BigUint) -> bool {
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

fn test_miller_rabin(upper_bound: &u32, trial_count: &u16) -> u32 {
    let mut dif_count = 0;

    for i in 3..*upper_bound {
        if monte_carlo(&BigUint::from(i), trial_count) != is_prime(&BigUint::from(i)) {
            dif_count += 1;
        }

    }

    dif_count
}

fn main() {
    let upper_bound = 100000;
    let trial_count = 10;
    let dif_count = test_miller_rabin(&upper_bound, &trial_count);
    println!("Out of {} integers, there were {} differences between Miller-Rabin and a deterministic algorithm.", upper_bound, dif_count);


    let start = Instant::now();
    let prime_str = b"49790921912819110019003521637763748399072771256062128988437189616228355821145834783451215869998723492323628198577054239101181556609916127864608488018093426129641387774385490891035446702272744866010729";
    let suspect_prime = BigUint::parse_bytes(prime_str, 10).unwrap();

    if monte_carlo(&suspect_prime, &trial_count) {
        println!("{}-digit prime calculated successfully", &prime_str.len());
    } else {
        println!("Suspected prime was not actually prime.")
    }

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}

