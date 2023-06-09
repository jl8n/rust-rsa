pub use crate::primes::is_prime::monte_carlo;
use num_bigint::BigInt;
use num_traits::{One, Zero};
use std::fs;

/// Encode a base-26 string to a base-10 integer
pub fn alphabet_to_base10(s: &String) -> BigInt {
    let base10_str: String = s
        .chars()
        .filter(|c| c.is_alphabetic()) // strip non-alphabet chars
        .map(|c| c.to_ascii_lowercase() as u32 - 96) // convert to ascii / subtract 96 so a == 1, z == 26
        .map(|n| n.to_string())
        .collect();
    let byte_str = base10_str.as_bytes();
    BigInt::parse_bytes(byte_str, 10).unwrap()
}

/// Convert a sufficiently large number to a nearby prime of a set length
pub fn nearby_prime(mut n: BigInt, length: u32) -> BigInt {
    // Make `n` a set length via modulus
    let mask_base = BigInt::from(10u8);
    let mask = mask_base.pow(length);
    n = n % mask;

    // Make `n` odd if it's even by adding 1
    if &n & BigInt::one() == BigInt::zero() {
        n += BigInt::one();
    }

    let mut is_prime = false;

    // Add 2 until `n` is prime
    while !is_prime {
        if !monte_carlo(&n, &10) {
            n += BigInt::from(2u8);
        } else {
            is_prime = true;
        }
    }

    n
}

pub fn generate_prime(seed: &String, length: u32) -> BigInt {
    let pre_prime = alphabet_to_base10(&seed);
    let prime = nearby_prime(pre_prime, length);
    prime
}

pub fn read_file(file_path: &str) -> String {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    contents
}
