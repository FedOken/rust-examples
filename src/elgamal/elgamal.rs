extern crate rand;

use num_bigint::{BigInt, BigUint};
use num_traits::{Zero, ToPrimitive};
use num_primes::{Generator, Verification, BigUint as BigUintPrime};
use num_traits::One;
use rand::Rng;
use num_integer::gcd;
use hex;

pub fn hex_to_number(hex: String) -> BigInt {
    let hex: Vec<u8> = hex::decode(hex).expect("Decoding failed");
    BigInt::from(BigUint::from_bytes_be(&hex))
}

pub fn genereate_keys(p_bits_from: usize, p_bits_to: usize) -> (BigInt, BigInt, BigInt, BigInt) {
    let p = generate_prime_number(p_bits_from, p_bits_to);
    let g = generate_primitive_root(&p);
    let private_key = generate_number_in_range(&2, &(p.to_usize().unwrap() - 1));
    let public_key = g.modpow(&private_key, &p);

    return (BigInt::from(p), BigInt::from(g), BigInt::from(private_key), BigInt::from(public_key));
}

// Select: k, where 1 < k < p - 1
// a = g^k mod p
// b = y^k*H(m) mod p
// Where y: public_key
// Where H(m): hash message in BigUint representation
pub fn encode(hex_num: &BigInt, p: &BigInt, g: &BigInt, public_key: &BigInt) -> (BigInt, BigInt) {
    let one = BigInt::from(1u32);
    let p_minus_one = p - &one;

    // k = [2, p - 1)
    let k = BigInt::from(generate_number_in_range(&2, &p_minus_one.to_usize().unwrap()));
    let a = g.modpow(&k, p);
    let b = (public_key.pow(k.to_u32().unwrap()) * hex_num) % p;

    return (a, b);
}

// H(m) = b(a^x)^(-1) mod p = b * a^(p - 1 - x) mod p
// Where x: private_key
// Where H(m): hash message in BigUint representation
pub fn decode(a: &BigInt, b: &BigInt, p: &BigInt, private_key: &BigInt) -> BigInt {
    b * a.pow(p.to_u32().unwrap() - 1 - private_key.to_u32().unwrap()) % p
}

pub fn sign(hex_num: &BigInt, p: &BigInt, g: &BigInt, private_key: &BigInt) -> (BigInt, BigInt) {
    let one = BigInt::from(1u32);
    let p_minus_one = p - &one;

    let mut k = BigInt::from(generate_number_in_range(&2, &p_minus_one.to_usize().unwrap()));
    while gcd(k.clone(), p_minus_one.clone()) != one {
        k = BigInt::from(generate_number_in_range(&2, &p_minus_one.to_usize().unwrap()));
    }
    let k_inverse_modulo = BigInt::from(calculate_inverse_modulo(k.to_i64().unwrap(), p_minus_one.to_i64().unwrap()) as u64);

    let r = g.modpow(&k, p);
    let s = ((hex_num - private_key * &r) * k_inverse_modulo).modpow(&one, &p_minus_one);

    return (r, s)
}

// y^r * r^s mod p = g^m mod p
// Where y: public_key

pub fn verify_sign(hex_num: &BigInt, p: &BigInt, g: &BigInt, r: &BigInt, s: &BigInt, public_key: &BigInt) -> bool {
    let left = (public_key.pow(r.try_into().unwrap()) * r.pow(s.try_into().unwrap())) % p;
    let right = g.modpow(hex_num, p);

    return left == right;
}

fn generate_prime_number(p_bits_from: usize, p_bits_to: usize) -> BigUint {
    // Generate 'p' bits length
    let mut bits = 0;
    while bits == 0 || bits % 8 != 0 {
        bits = rand::thread_rng().gen_range(p_bits_from..=p_bits_to);
    }

    // Generate 'p' value with 'bits' length
    let mut prime: BigUintPrime = Zero::zero();
    while prime.is_zero() || !Verification::is_prime(&prime) {
        prime = Generator::new_prime(bits);
    }

    BigUint::from(prime.to_u64().unwrap())
}

fn generate_primitive_root(p: &BigUint) -> BigUint {
    let one = BigUint::one();
    let p_minus_one = p - &one;

    let mut i = 0;
    let mut g = BigUint::from(2u32);
    let mut g_vec: Vec<BigUint> = Vec::new();
    while g < p_minus_one {
        if i == 5 {
            break;
        }

        if is_primitive_root(&g, &p) {
            g_vec.push(g.clone());
            i += 1
        }

        g += &one;
    }

    if g_vec.len() == 0 {
        panic!("Primitive root isn't found");
    }

    let g_rand = &g_vec[rand::thread_rng().gen_range(0..g_vec.len())];
    g_rand.clone()
}

fn is_primitive_root(g: &BigUint, p: &BigUint) -> bool {
    let one = BigUint::one();

    // Проверка, что g и p взаимно просты
    if gcd(g.clone(), p.clone()) != one {
        return false;
    }

  
    let p_minus_one = p - &one;
    if g.modpow(&p_minus_one, p) != one {
        return false;
    }

    let mut exp = BigUint::one();
    let mut res = BigUint::zero();
    while exp < p.clone() {
        res = g.modpow(&exp, p);

        if res == one && exp != p_minus_one {
            return false;
        }

        exp += &one;
    }

    res == one
}

fn generate_number_in_range(from: &usize, to: &usize) -> BigUint {
    BigUint::from(rand::thread_rng().gen_range(*from..*to))
}


// Realization for Euclid's algorithm
fn calculate_inverse_modulo(k: i64, p: i64) -> i64 {
    let (_, x, _) = extended_euclidean_algorithm(k, p);

    let result = (x % p + p) % p;

    result
}

fn extended_euclidean_algorithm(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        return (a, 1, 0);
    }

    let (d, x1, y1) = extended_euclidean_algorithm(b, a % b);
    let x = y1;
    let y = x1 - (a / b) * y1;

    (d, x, y)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all() {
        let hex_num = hex_to_number("1a2b".to_string());
        println!("hex_num: {}", hex_num);
        println!("");

        let (p, g, private_key, public_key) = genereate_keys(16, 16);

        println!("p: {}", p);
        println!("g: {}", g);
        println!("Private key: {}", private_key);
        println!("Public key: {}", public_key);
        println!("");

      
        let (a, b) = encode(&hex_num, &p, &g, &public_key);
        println!("a: {}", a);
        println!("b: {}", b);
       

        let m = decode(&a, &b, &p, &private_key);
        println!("m: {}", m);
        println!("");

        assert_eq!(hex_num, m);

        // let hex_num = BigInt::from(3u32);
        // let p = BigInt::from(23u32);
        // let g = BigInt::from(5u32);
        // let private_key = BigInt::from(7u32);
        // let public_key = BigInt::from(17u32);

        let (r, s) = sign(&hex_num, &p, &g, &private_key);
        println!("r: {}", r);
        println!("s: {}", s);
        println!("");

        let verify_result = verify_sign(&hex_num, &p, &g, &r, &s, &public_key);
        assert_eq!(verify_result, true);

        let r = BigInt::from(1u32);
        let verify_result = verify_sign(&hex_num, &p, &g, &r, &s, &public_key);
        assert_eq!(verify_result, false)
    }
}
// cargo test -- elgamal --nocapture