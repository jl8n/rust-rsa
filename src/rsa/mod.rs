use num_bigint::BigInt;
use num_traits::{One, Zero};
use pem::{encode, Pem};
use simple_asn1::{to_der, ASN1Block};

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

pub fn generate_rsa_key_components(p: &BigInt, q: &BigInt) -> (BigInt, BigInt, BigInt) {
    let n = p * q;
    let r = (p - BigInt::one()) * (q - BigInt::one()); // `r` is the totient phi(n)
    let e = BigInt::from(65537u32); // `e` is relatively prime with r

    let d = modular_inv(&e, &r).unwrap_or_else(|e| {
        println!("Error: {}", e);
        BigInt::from(0u8)
    }); // d = inverse of `e mod r`

    (n, e, d)
}

fn encode_public_key(n: &BigInt, e: &BigInt) -> Vec<u8> {
    let public_key = vec![
        ASN1Block::Integer(0, n.clone()),
        ASN1Block::Integer(0, e.clone()),
    ];
    to_der(&ASN1Block::Sequence(0, public_key)).unwrap()
}

fn encode_private_key(n: &BigInt, d: &BigInt) -> Vec<u8> {
    let private_key = vec![
        ASN1Block::Integer(0, n.clone()),
        ASN1Block::Integer(0, d.clone()),
    ];
    to_der(&ASN1Block::Sequence(0, private_key)).unwrap()
}

pub fn write_keys_to_pem_files(n: &BigInt, e: &BigInt, d: &BigInt) {
    let public_key_pem = Pem::new("RSA PUBLIC KEY", encode_public_key(n, e));
    let private_key_pem = Pem::new("RSA PRIVATE KEY", encode_private_key(n, d));
    std::fs::write("public_key.pem", encode(&public_key_pem)).unwrap();
    std::fs::write("private_key.pem", encode(&private_key_pem)).unwrap();
}
