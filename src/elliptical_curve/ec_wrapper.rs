use num_bigint::{BigUint,BigInt};
use hex;

#[cfg(test)]
use std::collections::HashMap;

pub fn hex_to_number(hex: String) -> BigInt {
    let hex: Vec<u8> = hex::decode(hex).expect("Decoding failed");
    BigInt::from(BigUint::from_bytes_be(&hex))
}

pub fn calculate_inverse_modulo(k: BigInt, p: BigInt) -> BigInt {
    let (_, x, _) = extended_euclidean_algorithm(k, p.clone());

    let result = (x % &p + &p) % &p;

    result
}

pub fn extended_euclidean_algorithm(a: BigInt, b: BigInt) -> (BigInt, BigInt, BigInt) {
    let zero = BigInt::from(0);
    let one = BigInt::from(1);

    if b == zero {
        return (a, one, zero);
    }

    let (d, x1, y1) = extended_euclidean_algorithm(b.clone(), a.clone() % b.clone());
    let x = y1.clone();
    let y = x1 - (&a / &b) * y1;

    (d, x, y)
}

// y^2 = x^3 + ax + b (mod p)
#[cfg(test)]
#[derive(Clone)]
struct ECurve {
    a: BigInt,
    b: BigInt,
    p: BigInt,
}

#[cfg(test)]
impl ECurve {
    pub fn create(a: BigInt, b: BigInt, p: BigInt) -> Self {
        Self { a, b, p }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct ECPoint {
    x: BigInt,
    y: BigInt,
    curve: ECurve,
}

#[cfg(test)]
impl ECPoint {
    pub fn create(x: BigInt, y: BigInt, curve: ECurve) -> Self {
        Self { x, y, curve }
    }

    pub fn is_point_on_curve(&self) -> bool {
        let one: BigInt = BigInt::from(1);

        let x = &self.x;
        let y = &self.y;

        let a = &self.curve.a;
        let b = &self.curve.b;
        let p = &self.curve.p;

        let left = y.pow(2).modpow(&one, p);
        let right = (x.pow(3) + a * x + b).modpow(&one, p);

        left.eq(&right)
    }

    pub fn add_point(&self, point: &ECPoint) -> Self {
        let one: BigInt = BigInt::from(1);

        let p = &self.curve.p;
        let a = &self.curve.a;

        let x1 = &self.x;
        let y1 = &self.y;

        let x2 = &point.x;
        let y2 = &point.y;

        let mut num = (y2 - y1).modpow(&one, p);
        let mut den = (x2 - x1).modpow(&BigInt::from(p - 2), p);
        if x1.eq(x2) && y1.eq(y2) {
            num = (BigInt::from(3) * (x1.pow(2)) + a).modpow(&one, p);
            den = (BigInt::from(2) * y1).modpow(&BigInt::from(p - 2), p);
        }
        let m = (num * den).modpow(&one, p);

        let x3 = (m.pow(2) - x1 - x2).modpow(&one, p);
        let y3 = (m * (x1 - &x3) - y1).modpow(&one, p);

        let new_point = ECPoint::create(x3, y3, self.curve.clone());
        if !new_point.is_point_on_curve() {
            panic!("New point isn't on curve")
        }

        new_point
    }

    pub fn multiply_point(&self, scalar: BigInt) -> Self {
        let one: BigInt = BigInt::from(1);

        if scalar == one {
            return self.clone();
        }

        let mut multiply_result = self.clone();


        let mut multiply_results: HashMap<BigInt, ECPoint> = HashMap::new();
        multiply_results.insert(one.clone(), self.clone());

        let mut multiplier = BigInt::from(2);
    
        while multiplier <= scalar {
            multiply_result = multiply_result.add_point(&multiply_result);

            multiply_results.insert(multiplier.clone(), multiply_result.clone());

            multiplier = multiplier * 2;
        }

        let mut multiply_results_sorted: Vec<(&BigInt, &ECPoint)> = multiply_results.iter().collect();
        multiply_results_sorted.sort_by_key(|a| -a.0);


        multiplier = multiply_results_sorted[0].0.clone();
        for (key, point) in multiply_results_sorted.iter().skip(1) {
            let multiplier_new = &multiplier + key.clone();
            if multiplier_new > scalar {
                continue;
            }

            multiply_result = multiply_result.add_point(&point);
            multiplier = multiplier_new;
        }
    
        multiply_result
    }

    pub fn sign(&self, d: BigInt, k: BigInt, z: BigInt) -> (BigInt, BigInt) {
        let one: BigInt = BigInt::from(1);
        let g = self.clone();

        let kg = g.multiply_point(k.clone());
        let r = kg.x.modpow(&one, &g.curve.p);
        let k_pow_minus_one = calculate_inverse_modulo(k.clone(), g.curve.p.clone());
        let s = (k_pow_minus_one * (z + &r * d)) % g.curve.p;

        return (r, s)
    }

    pub fn sign_verify(&self, r: BigInt, s: BigInt, z: BigInt, q: ECPoint) -> bool {
        let g = self.clone();

        let w = calculate_inverse_modulo(s.clone(), g.curve.p.clone()) % &g.curve.p;
        let u1 = (z * &w) % &g.curve.p;
        let u2 = (&r * &w) % &g.curve.p;

        let u1g = g.multiply_point(u1.clone());
        let u2q = q.multiply_point(u2.clone());

        let signature_point = u1g.add_point(&u2q);

        return r == signature_point.x % &g.curve.p;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_points() {
        // https://learn.ztu.edu.ua/pluginfile.php/196084/mod_resource/content/1/%D0%9B%D0%B5%D0%BA%D1%86%D1%96%D1%8F12.pdf

        // y^2 = x^3 + a*x + b (mod p)
        // y^2 = x^3 + x + 1 (mod 23)
        let curve = ECurve::create(BigInt::from(1), BigInt::from(1), BigInt::from(23));

        // P(3,10); Q(9,7); P+Q = R(17,20)
        let p = ECPoint::create(BigInt::from(3), BigInt::from(10), curve.clone());
        let q = ECPoint::create(BigInt::from(9), BigInt::from(7), curve.clone());
        let r = p.add_point(&q);
        assert_eq!(r.x, BigInt::from(17));
        assert_eq!(r.y, BigInt::from(20));

        // P(12,19); Q(5,4); P+Q = R(12,4)
        let p = ECPoint::create(BigInt::from(12), BigInt::from(19), curve.clone());
        let q = ECPoint::create(BigInt::from(5), BigInt::from(4), curve.clone());
        let r = p.add_point(&q);
        assert_eq!(r.x, BigInt::from(12));
        assert_eq!(r.y, BigInt::from(4));

        // P(5,4); P*P = R(17,20)
        let p = ECPoint::create(BigInt::from(5), BigInt::from(4), curve.clone());
        let p2 = p.add_point(&p);
        assert_eq!(p2.x, BigInt::from(17));
        assert_eq!(p2.y, BigInt::from(20));

        // P(5,4); 2*P+P = R(13,16)
        let p3 = p.add_point(&p2);
        assert_eq!(p3.x, BigInt::from(13));
        assert_eq!(p3.y, BigInt::from(16));

        // P(5,4); 2*P = R(17,20)
        let p_mul_2 = p.multiply_point(BigInt::from(2));
        assert_eq!(p_mul_2.x, BigInt::from(17));
        assert_eq!(p_mul_2.y, BigInt::from(20));

        // P(5,4); 3*P = R(13,16)
        let p_mul_3 = p.multiply_point(BigInt::from(3));
        assert_eq!(p_mul_3.x, BigInt::from(13));
        assert_eq!(p_mul_3.y, BigInt::from(16));

        // Check is P on curve
        let p = ECPoint::create(BigInt::from(4), BigInt::from(4), curve.clone());
        assert_eq!(p.is_point_on_curve(), false);
    }

    #[test]
    fn test_diffie_hellman() {
        // https://learn.ztu.edu.ua/pluginfile.php/196084/mod_resource/content/1/%D0%9B%D0%B5%D0%BA%D1%86%D1%96%D1%8F12.pdf

        // y^2 = x^3 + a*x + b (mod p)
        // y^2 = x^3 + -2 * x + 15 (mod 23)
        let curve = ECurve::create(BigInt::from(-2), BigInt::from(15), BigInt::from(23));

        // g(4,5)
        let g = ECPoint::create(BigInt::from(4), BigInt::from(5), curve.clone());

        let user_a_private_key = BigInt::from(3);
        let user_b_private_key = BigInt::from(7);

        // <user_a_private_key>*G = <user_a_public_key>(13,22)
        let user_a_public_key = g.multiply_point(user_a_private_key.clone());
        assert_eq!(user_a_public_key.x, BigInt::from(13));
        assert_eq!(user_a_public_key.y, BigInt::from(22));

        // <user_b_private_key>*G = <user_b_public_key>(13,22)
        let user_b_public_key = g.multiply_point(user_b_private_key.clone());
        assert_eq!(user_b_public_key.x, BigInt::from(17));
        assert_eq!(user_b_public_key.y, BigInt::from(8));

        // Main assertion
        // <user_b_public_key>*<user_a_private_key> = <user_a_private_key_diffie>(15,5)
        let user_a_private_key_diffie = user_b_public_key.multiply_point(user_a_private_key);
        assert_eq!(user_a_private_key_diffie.x, BigInt::from(15));
        assert_eq!(user_a_private_key_diffie.y, BigInt::from(5));

        // <user_A_public_key>*<user_b_private_key> = <user_b_private_key_diffie>(15,5)
        let user_b_private_key_diffie = user_a_public_key.multiply_point(user_b_private_key);
        assert_eq!(user_b_private_key_diffie.x, BigInt::from(15));
        assert_eq!(user_b_private_key_diffie.y, BigInt::from(5));
    }

    #[test]
    fn test_ecdsa_sign() {
        // https://learn.ztu.edu.ua/pluginfile.php/196084/mod_resource/content/1/%D0%9B%D0%B5%D0%BA%D1%86%D1%96%D1%8F12.pdf

        //// Input params
        // y^2 = x^3 + a*x + b (mod p)
        // y^2 = x^3 + -2 * x + 15 (mod 23)
        let curve = ECurve::create(BigInt::from(-2), BigInt::from(15), BigInt::from(23));
        // g(4,5)
        let g = ECPoint::create(BigInt::from(4), BigInt::from(5), curve.clone());

        let hex_string = hex::encode("Hello World!");
        let private_key = BigInt::from(3);
        let k = BigInt::from(19);
        //// End

        let hex_num = hex_to_number(hex_string);

        let public_key: ECPoint = g.multiply_point(private_key.clone());
        assert_eq!(public_key.x, BigInt::from(13));
        assert_eq!(public_key.y, BigInt::from(22));

        let (r, s) = g.sign(private_key, k, hex_num.clone());
        assert_eq!(r, BigInt::from(9));
        assert_eq!(s, BigInt::from(19));

        let is_valid = g.sign_verify(r, s, hex_num.clone(), public_key);
        assert_eq!(is_valid, true);
    }
}

// cargo test -- ec_wrapper --nocapture