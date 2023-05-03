use crate::primes::generate_prime::{generate_prime, read_file};
use std::time::Instant;

mod primes {
    pub mod generate_prime;
    pub mod is_prime;
}

#[cfg(test)]
mod tests;

fn main() {
    let start = Instant::now();
    let text = read_file("src/data/string1.txt");
    let prime = generate_prime(&text, 200);
    println!("Generated prime: {}", prime);

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}
