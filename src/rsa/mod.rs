use num_bigint::BigInt;
use num_traits::{One, Zero};
use std::mem;

pub fn extended_euclid(b: &BigInt, a: &BigInt) -> (BigInt, BigInt, BigInt) {
    let (x0, x1) = (BigInt::one(), BigInt::zero());
    let (y0, y1) = (BigInt::zero(), BigInt::one());
    let mut new_a = a.clone();
    let mut new_b = b.clone();

    while a != &BigInt::zero() {
        let gcd = &new_b / &new_a;
        mem::swap(&mut new_a, &mut new_b);
        new_a = &new_b % &new_a;

        // let (x0, x1) = (x1, x0 - gcd * x1);
        // let (y0, y1) = (y1, y0 - gcd * y1);
    }

    (new_b, x0, y0)
}

// pub fn extended_euclid(a: &BigUint, b: &BigUint) -> (BigUint, BigUint, BigUint) {
//     let gcd = a.clone();
//     let x = BigUint::one();
//     let y = BigUint::zero();

//     if b == &BigUint::zero() {
//         return (gcd, x, y);
//     } else {
//         let (gcd, x, y) = extended_euclid(b, &(a % b));
//         return (gcd, y, x - (a / b) * &y);
//     }
// }

pub fn modular_inv(a: &BigInt, m: &BigInt) -> Result<BigInt, String> {
    // g = gratest common divisor of `a` and `m`
    // x, _ = Bezout coefficients of `a` and `m`
    let (gcd, x, _) = extended_euclid(&a, &m);

    if gcd != BigInt::one() {
        // g is not 1, so modular inverse doesn't exist
        Err(String::from("Modular inverse does not exist"))
    } else {
        // x may be negative, so add m to make it positive
        Ok(x % BigInt::from(m.clone()))
    }
}

pub fn generate_key(p: &BigInt, q: &BigInt) {
    let n = p * q;
    let r = (p - BigInt::one()) * (q - BigInt::one()); // totient phi(n)
    let e = BigInt::from(65537u32); // relatively prime with r

    let d = modular_inv(&e, &r).unwrap_or_else(|e| {
        println!("Error: {}", e);
        BigInt::from(0u8)
    });

    println!("n: {}, d: {}", n, d);
}
