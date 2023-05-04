use num_bigint::{BigInt, BigUint};
use num_traits::{One, Zero};

pub fn extended_euclid(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
    if a == &BigInt::zero() {
        (b.clone(), BigInt::zero(), BigInt::one())
    } else {
        let (gcd, x, y) = extended_euclid(&(b % a), a);
        (gcd, y - (b / a) * &x, x)
    }
}

pub fn modular_inv(a: &BigInt, m: &BigInt) -> Result<BigInt, String> {
    // g = gratest common divisor of `a` and `m`
    // x, _ = Bezout coefficients of `a` and `m`
    let (gcd, x, _) = extended_euclid(&a, &m);

    if gcd != BigInt::one() {
        Err(String::from("Modular inverse does not exist"))
    } else {
        // Compute the modular inverse of `a` (mod `m`) as `x + m` if `x` is negative
        let inv = if x < BigInt::zero() { x + m } else { x };
        Ok(inv)
    }
}

pub fn generate_keys(p: &BigInt, q: &BigInt) -> (BigInt, BigInt, BigInt) {
    let n = p * q;
    let r = (p - BigInt::one()) * (q - BigInt::one()); // `r` is the totient phi(n)
    let e = BigInt::from(65537u32); // `e` is relatively prime with r

    let d = modular_inv(&e, &r).unwrap_or_else(|e| {
        println!("Error: {}", e);
        BigInt::from(0u8)
    }); // d = inverse of `e mod r`

    (n, e, d)
}
