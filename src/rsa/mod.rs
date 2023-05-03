use num_bigint::BigUint;
use num_traits::{One, Zero};

pub fn extended_euclid(b: &BigUint, a: &BigUint) -> (BigUint, BigUint, BigUint) {
    if a == &BigUint::zero() {
        return (b.clone(), BigUint::zero(), BigUint::one());
    }

    let (gcd, x1, y1) = extended_euclid(&(b % a), a);

    let x = &y1 - &(b / a) * &x1;
    let y = x1;

    (gcd, x, y)
}

pub fn modular_inv(a: &BigUint, m: &BigUint) -> Result<BigUint, String> {
    // g = gratest common divisor of `a` and `m`
    // x, _ = Bezout coefficients of `a` and `m`
    let (g, x, _) = extended_euclid(&a, &m);

    if g != BigUint::from(1u8) {
        // g is not 1, so modular inverse doesn't exist
        Err(String::from("Modular inverse does not exist"))
    } else {
        // x may be negative, so add m to make it positive
        Ok(x % m)
    }
}

pub fn generate_key(p: &BigUint, q: &BigUint) {
    let n = p * q;
    let r = (p - BigUint::one()) * (q - BigUint::one()); // totient phi(n)
    let e = BigUint::from(65537u32); // relatively prime with r

    let d = modular_inv(&e, &r).unwrap_or_else(|e| {
        println!("Error: {}", e);
        BigUint::from(0u8)
    });

    println!("n: {}, d: {}", n, d);
}
