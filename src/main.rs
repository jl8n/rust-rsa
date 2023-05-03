use crate::{
    primes::generate_prime::{generate_prime, read_file},
    rsa::generate_key,
};
use std::time::Instant;

mod primes {
    pub mod generate_prime;
    pub mod is_prime;
}
mod rsa;

#[cfg(test)]
mod tests;

fn main() {
    let start = Instant::now();
    let text1 = read_file("src/data/string1.txt");
    let text2 = read_file("src/data/string2.txt");
    let prime1 = generate_prime(&text1, 200);
    let prime2 = generate_prime(&text2, 200);

    generate_key(&prime1, &prime2);

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}
