use num_bigint::BigUint;
use std::time::Instant;
use rust_rsa;


fn main() {
    let start = Instant::now();
    let trial_count = 10;
    let prime_str = b"48950413500958247776918939895110243197568110749207986981599466255471171998956583861825130114208146376913862645586256012246129830589390593869305615234266606932439325212639436201253190830710923034595811";
    let suspect_prime = BigUint::parse_bytes(prime_str, 10).unwrap();

    if rust_rsa::monte_carlo(&suspect_prime, &trial_count) {
        println!("{}-digit prime calculated successfully", &prime_str.len());
    } else {
        println!("Suspected prime was not actually prime.")
    }

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}

